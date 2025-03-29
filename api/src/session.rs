use crate::model::session::{
    MessageApiTask, MessageParticipantTask, MessageTaskParticipant, Session, TypeMessage,
};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::task;
use uuid::Uuid;

impl Session {
    pub fn new(participants: Vec<Uuid>, filters: Vec<String>, id: Uuid) -> Arc<Session> {
        let (_, tx) = unbounded_channel::<TypeMessage>();
        let (rx, _) = broadcast::channel::<MessageTaskParticipant>(100);

        let session = Arc::new(Self {
            id,
            participants,
            filters,
            tx,
            rx,
        });

        let session_clone = session.clone();

        task::spawn(async move {
            session_clone.worker().await;
        });

        session
    }

    pub async fn worker(self: &Arc<Self>) {
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

        while let Ok(message) = self.tx.recv().await {
            // If some participant are not yet connected
            if users_connection_state.len() != self.participants.len() {
                match message {
                    // If a participant joins the session
                    TypeMessage::Api(MessageApiTask::Join { user_id, tx }) => {
                        users_senders.insert(user_id, tx);
                        users_connection_state.insert(user_id, true);

                        // Informs all connected participants that a new participant has connected
                        _ = self.rx.send(MessageTaskParticipant::UserJoined(user_id));

                        // If all participants are connected
                        if users_connection_state.len() == self.participants.len() {
                            self.add_movie(&movies, &votes, 2);
                            _ = self.rx.send(MessageTaskParticipant::Content(vec![
                                *movies.get(0).unwrap(),
                                *movies.get(1).unwrap(),
                            ]));
                        }
                    }

                    // If a participant leaves the session
                    TypeMessage::Api(MessageApiTask::Leave(user_id)) => {
                        users_senders.remove(&user_id);
                        users_connection_state.remove(&user_id);
                        _ = self.rx.send(MessageTaskParticipant::UserLeaved(user_id));
                    }
                    _ => {}
                }
            } else {
                match message {
                    TypeMessage::Participant { user_id, message } => {
                        match message {
                            // User vote
                            MessageParticipantTask::Vote(user_vote) => {
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
                                                            _ = self.rx.send(
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

                                            if !is_match {
                                                if (movies.len() - 1)
                                                    == (*position - global_position)
                                                {
                                                    self.add_movie(&movies, &votes, 1);
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
                                nb_restart_demand += 1;
                                if nb_restart_demand > (self.participants.len() / 2) {
                                    _ = self.rx.send(MessageTaskParticipant::Restarted);
                                    movies.clear();
                                    global_position = 0;
                                    users_positions =
                                        users_positions.into_iter().map(|(k, _)| (k, 0)).collect();
                                    self.add_movie(&movies, &votes, 2);
                                    _ = self.rx.send(MessageTaskParticipant::Content(vec![
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
                                users_connection_state.insert(user_id, true);

                                // Informs all connected participants that a new participant has connected
                                _ = self.rx.send(MessageTaskParticipant::UserJoined(user_id));

                                match users_positions.get_mut(&user_id) {
                                    Some(position) => {
                                        _ = users_senders.get(&user_id).unwrap().send(
                                            MessageTaskParticipant::Content(vec![
                                                *movies.get(*position - global_position).unwrap(),
                                            ]),
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            MessageApiTask::Leave(user_id) => {
                                users_connection_state.insert(user_id, false);

                                // Informs all connected participants that a participant has disconnected
                                _ = self.rx.send(MessageTaskParticipant::UserLeaved(user_id));
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn add_movie(
        mut movies: VecDeque<Uuid>,
        mut votes: VecDeque<HashMap<Uuid, bool>>,
        nb: usize,
    ) {
        for i in 0..nb {
            movies.push_back(Uuid::new_v4());
            votes.push_back(HashMap::new());
        }
    }
}
