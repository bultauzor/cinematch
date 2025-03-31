use crate::db::DbHandler;
use crate::model::recommendation::{Recommendation, RecommendationView};
use crate::recommender::Error;
use crate::recommender::cinematch::RecommenderCommand;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

struct InnerRecommendationsIterator {
    min: i64,
    max: i64,
    inf: i64,
    sup: i64,
    data: VecDeque<Recommendation>,
}

pub struct RecommendationsIterator {
    recommendation_id: Uuid,
    tx: mpsc::UnboundedSender<RecommenderCommand>,
    db_handler: DbHandler,
    inner: Arc<Mutex<InnerRecommendationsIterator>>,
}

#[allow(dead_code)]
impl RecommendationsIterator {
    pub(super) async fn new(
        recommendation_id: Uuid,
        tx: mpsc::UnboundedSender<RecommenderCommand>,
        db_handler: DbHandler,
    ) -> Result<Self, Error> {
        _ = tx.send(RecommenderCommand::ArcInc(recommendation_id));

        let inner = Self::load_inner(&recommendation_id, 0, &db_handler).await?;

        Ok(Self {
            recommendation_id,
            tx,
            db_handler,
            inner: Arc::new(Mutex::new(inner)),
        })
    }

    pub async fn get(&self, pos: usize) -> Result<Option<RecommendationView>, Error> {
        let pos = pos as i64;
        let mut inner = self.inner.lock().await;

        if pos < inner.min || pos > inner.max {
            return Ok(None);
        }

        if pos < inner.inf || pos > inner.sup {
            *inner = Self::load_inner(&self.recommendation_id, pos, &self.db_handler).await?;
        }

        let r = inner.data.get((pos - inner.min) as usize);
        Ok(r.cloned().map(|r| r.into()))
    }

    pub async fn len(&self) -> i64 {
        self.inner.lock().await.max
    }

    pub async fn next(&self) -> Result<Option<RecommendationView>, Error> {
        let inf = self.inner.lock().await.inf;
        let res = self.get(inf as usize).await?;
        if res.is_some() {
            let mut inner = self.inner.lock().await;
            inner.inf += 1;
            inner.data.pop_front();
        }

        Ok(res)
    }

    pub async fn add_back(&self, n: usize) -> Result<(), Error> {
        let mut inner = self.inner.lock().await;

        if inner.sup < inner.max {
            let n = (n as i64).min(inner.max - inner.sup);
            let recc = self
                .db_handler
                .fetch_recommendations_from(&self.recommendation_id, inner.sup + 1, n)
                .await?;
            inner.sup += recc.len() as i64;
            inner.data.extend(recc);
        }

        Ok(())
    }

    async fn load_inner(
        recommendation_id: &Uuid,
        pos: i64,
        db_handler: &DbHandler,
    ) -> Result<InnerRecommendationsIterator, Error> {
        let recc = db_handler
            .fetch_recommendations_from(recommendation_id, pos, 40)
            .await?;
        let (min, max, inf, sup) = match recc.first() {
            None => (0, 0, 0, 0),
            Some(r) => (1, r.o2, r.o1, r.o2.min(r.o1 + 40)),
        };

        let mut data = VecDeque::with_capacity(40);
        data.extend(recc);

        Ok(InnerRecommendationsIterator {
            min,
            max,
            inf,
            sup,
            data,
        })
    }
}

impl Drop for RecommendationsIterator {
    fn drop(&mut self) {
        _ = self
            .tx
            .send(RecommenderCommand::ArcDec(self.recommendation_id));
    }
}
