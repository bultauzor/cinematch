/* Pun intended */
use crate::db::DbHandler;
use crate::model::content::Content;
use crate::model::recommendation::RecommendationParametersInput;
use crate::provider::tmdb::TmdbProvider;
use crate::provider::{Provider, ProviderKey};
use crate::recommender::Error;
use crate::recommender::utils::RecommendationsIterator;
use chrono::Utc;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio::time::Instant;
use tracing::{error, info, warn};
use uuid::Uuid;

const CONTENT_RECOMMENDATION_LEVEL: u8 = 1;
const CHECK_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24 * 4); // 4 days

#[allow(clippy::large_enum_variant)]
enum UpdateCommand {
    UpdateRecommenderEmbeddings,
    UpdateContentRecommendations(Content),
}

pub(super) enum RecommenderCommand {
    ArcInc(Uuid),
    ArcDec(Uuid),
}

pub struct CinematchRecommender {
    db_handler: DbHandler,
    provider: Arc<TmdbProvider>,
    updater_tx: mpsc::UnboundedSender<UpdateCommand>,
    recommendation_tx: mpsc::UnboundedSender<RecommenderCommand>,
}

impl CinematchRecommender {
    pub async fn new(
        db_handler: DbHandler,
        provider: Arc<TmdbProvider>,
    ) -> Result<Arc<Self>, sqlx::Error> {
        let (updater_tx, updater_rx) = mpsc::unbounded_channel();
        let (recommendation_tx, recommendation_rx) = mpsc::unbounded_channel();

        let cr = Arc::new(Self {
            db_handler,
            provider,
            updater_tx,
            recommendation_tx,
        });

        cr.update_recommender_embeddings().await?;
        {
            let cr = cr.clone();
            tokio::spawn(cr.worker_updater(updater_rx));
        }
        {
            let cr = cr.clone();
            tokio::spawn(cr.worker_recommender(recommendation_rx));
        }

        Ok(cr)
    }

    async fn update_recommender_embeddings(self: &Arc<Self>) -> Result<(), sqlx::Error> {
        info!("Updating embeddings");
        self.db_handler.update_recommender_embeddings().await?;
        info!("Embeddings updated");

        Ok(())
    }

    async fn worker_updater(self: Arc<Self>, mut rx: mpsc::UnboundedReceiver<UpdateCommand>) {
        let mut content_queue = VecDeque::with_capacity(40);
        let mut update_content_set = JoinSet::new();

        let (embeddings_tx, mut embeddings_rx) = mpsc::unbounded_channel();
        let clone = self.clone();
        tokio::spawn(async move {
            let mut should_update = false;
            let itv = tokio::time::interval(Duration::from_secs(40));
            tokio::pin!(itv);

            loop {
                tokio::select! {
                    Some(_) = embeddings_rx.recv() => {
                        should_update = true;
                        itv.reset();
                        itv.tick().await;
                    }
                    _ = itv.tick(), if should_update => {
                        if let Err(error) = clone.update_recommender_embeddings().await {
                            warn!(?error, "UpdateRecommenderEmbeddings task failed");
                        }
                        should_update = false;
                    }
                }
            }
        });

        loop {
            tokio::select! {
                Some(command) = rx.recv() => {
                    match command {
                        UpdateCommand::UpdateRecommenderEmbeddings => {
                            _ = embeddings_tx.send(());
                        }
                        UpdateCommand::UpdateContentRecommendations(content) => {
                            content_queue.push_back(content);
                        }
                    }
                }
                res = update_content_set.join_next() => {
                    match res {
                        None if !content_queue.is_empty() => {
                            let clone = self.clone();
                            let content = content_queue.pop_front().unwrap();
                            update_content_set.spawn(async move {
                                clone.update_content_recommendations(content).await
                            });
                        }
                        Some(Ok(Err(error))) => {
                            warn!(?error, "UpdateContentRecommendations task failed");
                        }
                        _ => {}
                    }
                }
                else => return,
            }
        }
    }

    async fn worker_recommender(
        self: Arc<Self>,
        mut rx: mpsc::UnboundedReceiver<RecommenderCommand>,
    ) {
        let mut track = HashMap::new();
        let itv = tokio::time::interval(Duration::from_secs(600));
        tokio::pin!(itv);
        itv.tick().await;

        loop {
            tokio::select! {
                Some(command) = rx.recv() => {
                    match command {
                        RecommenderCommand::ArcInc(recommender_id) => {
                            _ = self
                                .db_handler
                                .inc_recommendation_arc(&recommender_id)
                                .await
                                .map_err(|error| error!(?error));
                            track.insert(recommender_id, Instant::now());
                        }
                        RecommenderCommand::ArcDec(recommender_id) => {
                            _ = self
                                .db_handler
                                .dec_recommendation_arc(&recommender_id)
                                .await
                                .map_err(|error| error!(?error));
                            track.insert(recommender_id, Instant::now());
                        }
                    }
                }
                _ = itv.tick() => {
                    if let Ok(outdated) = self.db_handler.get_recommendation_outdated().await {
                        for rec in outdated {
                            if rec.refcount <= 0 || (!track.contains_key(&rec.recommendation_id)
                                    || track.get(&rec.recommendation_id)
                                            .map(|inst| inst.elapsed() > Duration::from_secs(3600 * 24 * 2))
                                            .unwrap_or_default()) {
                                _ = self.db_handler
                                    .delete_recommendations(&rec.recommendation_id)
                                    .await
                                    .map_err(|error| error!(?error));
                            }
                        }
                    }
                }
            }
        }
    }

    // Let's help rust a little bit by twisting the future 😡
    #[allow(clippy::manual_async_fn)]
    fn rec_update_content_recommendations(
        self: Arc<Self>,
        content: Content,
        limit: u8,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        async move {
            if limit == 0 {
                return Ok(());
            }

            let updated_at = self
                .db_handler
                .get_recommender_providers_updated_at(&content.content_id)
                .await?;
            if updated_at + CHECK_INTERVAL < Utc::now().naive_utc() {
                let recc = self
                    .provider
                    .get_recommendations(&ProviderKey::from_str(&content.provider_id)?)
                    .await?;

                let mut contents = Vec::with_capacity(recc.len());
                for r in recc {
                    let r = match self
                        .db_handler
                        .get_content_by_provider_key(&r.provider_id)
                        .await?
                    {
                        Some(_) => {
                            let (content_id, updated_at) =
                                self.db_handler.update_content(&r).await?;
                            r.hydrate(content_id, updated_at)
                        }
                        None => {
                            let (content_id, updated_at) =
                                self.db_handler.insert_content(&r).await?;
                            r.hydrate(content_id, updated_at)
                        }
                    };

                    contents.push(r);
                }

                self.db_handler
                    .update_recommender_providers_rel(
                        &content.content_id,
                        contents.iter().map(|c| c.content_id).collect(),
                    )
                    .await?;

                let mut set = JoinSet::new();
                let mut itv = tokio::time::interval(Duration::from_millis(200));
                itv.tick().await;

                for content in contents {
                    let arc = self.clone();
                    set.spawn(async move {
                        arc.rec_update_content_recommendations(content, limit - 1)
                            .await
                    });

                    if set.len() == 20 {
                        set.join_next().await;
                        if limit == 1 {
                            itv.tick().await;
                        }
                    }
                }

                set.join_all().await;
            }

            Ok(())
        }
    }
    async fn update_content_recommendations(
        self: &Arc<Self>,
        content: Content,
    ) -> Result<(), Error> {
        info!("Updating content recommendations");
        let s = Instant::now();
        self.clone()
            .rec_update_content_recommendations(content, CONTENT_RECOMMENDATION_LEVEL + 1)
            .await
            .map_err(|err| {
                error!(error=?err, "Failed to update content recommendations");
                err
            })?;
        info!(duration=?s.elapsed(), "Content recommendations updated");

        Ok(())
    }

    pub async fn get_user_recommendation(
        self: &Arc<Self>,
        params: RecommendationParametersInput,
    ) -> Result<RecommendationsIterator, Error> {
        let params = match self
            .db_handler
            .get_recommendations_by_hash(&params.hash())
            .await?
        {
            Some(rec) => {
                if rec.updated_at + Duration::from_secs(600) < Utc::now().naive_utc() {
                    self.db_handler
                        .update_recommendations(&rec.recommendation_id)
                        .await?;
                }
                params.hydrate(rec.recommendation_id)
            }
            None => self.db_handler.create_recommendations(params).await?,
        };

        RecommendationsIterator::new(
            params.recommendation_id,
            self.recommendation_tx.clone(),
            self.db_handler.clone(),
        )
        .await
    }

    pub async fn rated_positively(self: &Arc<Self>, content_id: Uuid) -> Result<(), Error> {
        let content = self.db_handler.get_content_by_id(&content_id).await?;
        if let Some(content) = content {
            _ = self
                .updater_tx
                .send(UpdateCommand::UpdateContentRecommendations(content));
        }

        Ok(())
    }

    pub fn user_seen_changed(self: &Arc<Self>) {
        _ = self
            .updater_tx
            .send(UpdateCommand::UpdateRecommenderEmbeddings);
    }
}
