use crate::model::session::{
    MessageApiTask, MessageParticipantTask, MessageTaskParticipant, TypeMessage,
};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::task;
use tracing::info;
use uuid::Uuid;

pub struct Session {
    pub id: Uuid,
    pub participants: Vec<Uuid>,
    pub filters: Vec<String>,
    pub rx: broadcast::Receiver<MessageTaskParticipant>,
    pub tx: UnboundedSender<TypeMessage>,
}

impl Session {
    pub fn new(participants: Vec<Uuid>, filters: Vec<String>, id: Uuid) -> Arc<Session> {
        let (unbounded_tx, unbounded_rx) = unbounded_channel::<TypeMessage>();
        let (broadcast_tx, broadcast_rx) = broadcast::channel::<MessageTaskParticipant>(100);

        let session = Arc::new(Self {
            id,
            participants,
            filters,
            rx: broadcast_rx,
            tx: unbounded_tx,
        });

        let session_clone = session.clone();

        task::spawn(async move {
            session_clone.worker(broadcast_tx, unbounded_rx).await;
        });

        session
    }

    pub async fn worker(
        self: &Arc<Self>,
        broadcast_tx: broadcast::Sender<MessageTaskParticipant>,
        mut unbounded_rx: UnboundedReceiver<TypeMessage>,
    ) {
        // Recommendations
        let mut movies: VecDeque<Uuid> = VecDeque::new();

        // Votes : VecDeque<HashMap<user(Uuid),user_vote(bool)>>
        let mut votes: VecDeque<HashMap<Uuid, bool>> = VecDeque::new();

        // Users_connection_state : HashMap<user(Uuid),is_connect(bool)>
        let mut users_connection_state: HashMap<Uuid, bool> = HashMap::new();

        // Users_senders : HashMap<user(Uuid),sender(UnboundedSender<MessageTaskParticipant>)
        let mut users_senders: HashMap<Uuid, UnboundedSender<MessageTaskParticipant>> =
            HashMap::new();

        // Users_positions : HashMap<user(Uuid),user_position(usize)>
        let mut users_positions: HashMap<Uuid, usize> = HashMap::new();

        let mut global_position = 0;

        let mut nb_restart_demand = 0;

        while let Some(message) = unbounded_rx.recv().await {
            // If some participant are not yet connected
            if users_connection_state.len() != self.participants.len() {
                match message {
                    // If a participant joins the session
                    TypeMessage::Api(MessageApiTask::Join { user_id, tx }) => {
                        info!(?user_id, "Join");
                        users_senders.insert(user_id, tx);
                        users_connection_state.insert(user_id, true);
                        users_positions.insert(user_id, 0);

                        // Informs all connected participants that a new participant has connected
                        _ = broadcast_tx.send(MessageTaskParticipant::UserJoined(user_id));

                        // If all participants are connected
                        if users_connection_state.len() == self.participants.len() {
                            Session::add_movie(&mut movies, &mut votes, 2);
                            _ = broadcast_tx.send(MessageTaskParticipant::Content(vec![
                                *movies.get(0).unwrap(),
                                *movies.get(1).unwrap(),
                            ]));
                        }
                    }

                    // If a participant leaves the session
                    TypeMessage::Api(MessageApiTask::Leave(user_id)) => {
                        info!(?user_id, "Leave");
                        users_senders.remove(&user_id);
                        users_connection_state.insert(user_id, false);
                        _ = broadcast_tx.send(MessageTaskParticipant::UserLeaved(user_id));

                        // If all participants are leaved
                        if users_connection_state.values().all(|&v| !v) {
                            return;
                        }
                    }
                    _ => {}
                }
            } else {
                match message {
                    TypeMessage::Participant { user_id, message } => {
                        match message {
                            // User vote
                            MessageParticipantTask::Vote(user_vote) => {
                                info!(?user_id, "Vote");
                                match users_positions.get_mut(&user_id) {
                                    Some(position) => {
                                        if let Some(map) = votes.get_mut(*position) {
                                            map.insert(user_id, user_vote);
                                            *position += 1;

                                            let mut is_match = false;

                                            // If all participants voted for a movie
                                            if map.len() == self.participants.len() {
                                                // If all votes are true - it's a match
                                                if map.values().all(|&value| value) {
                                                    match movies.get(*position - global_position) {
                                                        Some(movie) => {
                                                            _ = broadcast_tx.send(
                                                                MessageTaskParticipant::Result(
                                                                    *movie,
                                                                ),
                                                            );
                                                            is_match = true;
                                                        }
                                                        _ => {}
                                                    }

                                                // If at least one vote is false
                                                } else {
                                                    movies.pop_front();
                                                    global_position += 1;
                                                }
                                            }

                                            info!(is_match);

                                            if !is_match {
                                                if (movies.len() - 1)
                                                    == (*position - global_position)
                                                {
                                                    Session::add_movie(&mut movies, &mut votes, 1);
                                                }

                                                // Send new content
                                                _ = users_senders.get(&user_id).unwrap().send(
                                                    MessageTaskParticipant::Content(vec![
                                                        *movies
                                                            .get(*position - global_position)
                                                            .unwrap(),
                                                    ]),
                                                );
                                            }
                                        }
                                    }
                                    None => {}
                                }
                            }
                            // User restart demand
                            MessageParticipantTask::Restart => {
                                info!(?user_id, "Restart");
                                nb_restart_demand += 1;
                                if nb_restart_demand > (self.participants.len() / 2) {
                                    _ = broadcast_tx.send(MessageTaskParticipant::Restarted);
                                    movies.clear();
                                    global_position = 0;
                                    users_positions =
                                        users_positions.into_iter().map(|(k, _)| (k, 0)).collect();
                                    Session::add_movie(&mut movies, &mut votes, 2);
                                    _ = broadcast_tx.send(MessageTaskParticipant::Content(vec![
                                        *movies.get(0).unwrap(),
                                        *movies.get(1).unwrap(),
                                    ]));
                                }
                            }
                        }
                    }
                    TypeMessage::Api(message) => {
                        match message {
                            MessageApiTask::Join { user_id, tx } => {
                                info!(?user_id, "Join");
                                users_connection_state.insert(user_id, true);

                                // Informs all connected participants that a new participant has connected
                                _ = broadcast_tx.send(MessageTaskParticipant::UserJoined(user_id));

                                match users_positions.get_mut(&user_id) {
                                    Some(position) => {
                                        _ = tx.send(MessageTaskParticipant::Content(vec![
                                            *movies.get(*position - global_position).unwrap(),
                                        ]));
                                    }
                                    _ => {}
                                }
                            }
                            MessageApiTask::Leave(user_id) => {
                                info!(?user_id, "Leave");
                                users_connection_state.insert(user_id, false);

                                // Informs all connected participants that a participant has disconnected
                                _ = broadcast_tx.send(MessageTaskParticipant::UserLeaved(user_id));
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn add_movie(
        movies: &mut VecDeque<Uuid>,
        votes: &mut VecDeque<HashMap<Uuid, bool>>,
        nb: usize,
    ) {
        for _ in 0..nb {
            movies.push_back(Uuid::new_v4());
            votes.push_back(HashMap::new());
        }
    }
}
