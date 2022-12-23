use crate::message::Message;
use tokio::sync::broadcast::{self, error::RecvError};

pub struct Subscription {
    pub(crate) topic_name: String,
    pub(crate) receiver: broadcast::Receiver<Message>,
}

impl Subscription {
    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    pub async fn receive(&mut self) -> Result<Message, RecvError> {
        self.receiver.recv().await
    }
}
