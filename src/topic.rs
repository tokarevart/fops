use crate::subscription::Subscription;
use std::borrow::Borrow;
use std::cmp::Ordering;
use tokio::sync::broadcast::{self, error::SendError};

#[derive(Clone)]
pub struct Topic<M: Clone> {
    name: String,
    sender: broadcast::Sender<M>,
}

impl<M: Clone> Topic<M> {
    pub(crate) fn new(name: String, capacity: usize) -> Self {
        Self {
            name,
            sender: broadcast::channel(capacity).0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn publish(&self, msg: M) -> Result<usize, SendError<M>> {
        self.sender.send(msg)
    }

    pub fn subscribe(&self) -> Subscription<M> {
        Subscription {
            topic_name: self.name.clone(),
            receiver: self.sender.subscribe(),
        }
    }
}

impl<M: Clone> PartialEq for Topic<M> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl<M: Clone> Eq for Topic<M> {}

impl<M: Clone> PartialOrd for Topic<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl<M: Clone> Ord for Topic<M> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl<M: Clone> Borrow<str> for Topic<M> {
    fn borrow(&self) -> &str {
        self.name.borrow()
    }
}
