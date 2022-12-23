use crate::{message::Message, subscription::Subscription};
use std::borrow::Borrow;
use std::cmp::Ordering;
use tokio::sync::broadcast::{self, error::SendError};

#[derive(Clone)]
pub struct Topic {
    name: String,
    sender: broadcast::Sender<Message>,
}

impl Topic {
    pub(crate) fn new(name: String, capacity: usize) -> Self {
        Self {
            name,
            sender: broadcast::channel(capacity).0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn publish(&self, msg: Message) -> Result<usize, SendError<Message>> {
        self.sender.send(msg)
    }

    pub fn subscribe(&self) -> Subscription {
        Subscription {
            topic_name: self.name.clone(),
            receiver: self.sender.subscribe(),
        }
    }
}

impl PartialEq for Topic {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Topic {}

impl PartialOrd for Topic {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for Topic {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Borrow<str> for Topic {
    fn borrow(&self) -> &str {
        self.name.borrow()
    }
}
