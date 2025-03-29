use tokio::sync::broadcast;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
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

#[derive(Clone)]
pub enum MessageParticipantTask {
    /// Parameters : vote(bool)
    Vote(bool),
    Restart,
}

#[derive(Clone)]
pub enum MessageTaskParticipant {
    Result(Uuid),
    UserJoined(Uuid),
    UserLeaved(Uuid),
    Restarted,
    Content(Vec<Uuid>),
}

pub struct Session {
    pub id: Uuid,
    pub participants: Vec<Uuid>,
    pub filters: Vec<String>,
    pub tx: UnboundedReceiver<TypeMessage>,
    pub rx: broadcast::Sender<MessageTaskParticipant>,
}
