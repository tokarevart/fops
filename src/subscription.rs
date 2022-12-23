use tokio::sync::broadcast::{self, error::RecvError};

pub struct Subscription<M: Clone> {
    pub(crate) topic_name: String,
    pub(crate) receiver: broadcast::Receiver<M>,
}

impl<M: Clone> Subscription<M> {
    pub fn topic_name(&self) -> &str {
        &self.topic_name
    }

    pub async fn receive(&mut self) -> Result<M, RecvError> {
        self.receiver.recv().await
    }
}
