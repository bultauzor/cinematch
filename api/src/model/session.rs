use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

#[derive(Clone)]
pub enum TypeMessage {
    /// Parameters : message(MessageApiTask)
    Api(MessageApiTask),
    /// Parameters : user_id(Uuid), message(MessageParticipantTask)
    Participant {
        user_id: Uuid,
        message: MessageParticipantTask,
    },
}

#[derive(Clone)]
pub enum MessageApiTask {
    /// Parameters : user_id(Uuid), tx(UnboundedSender<MessageTaskParticipant>)
    Join {
        user_id: Uuid,
        tx: UnboundedSender<MessageTaskParticipant>,
    },
    /// Parameters : user_id(Uuid)
    Leave(Uuid),
}

#[derive(Clone, Deserialize)]
pub enum MessageParticipantTask {
    /// Parameters : vote(bool)
    Vote(bool),
    Restart,
}

#[derive(Clone, Serialize)]
pub enum MessageTaskParticipant {
    Result(Uuid),
    UserJoined(Uuid),
    UserLeaved(Uuid),
    Restarted,
    Content(Vec<Uuid>),
}
