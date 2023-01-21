use crate::subscription::Subscription;
use std::cmp::Ordering;
use std::{borrow::Borrow, hash::Hash};
use tokio::sync::broadcast::{self, error::SendError};

pub const CAPACITY: usize = 1;

#[derive(Clone, Debug)]
pub struct Topic<M: Clone> {
    name: Vec<u8>,
    sender: broadcast::Sender<M>,
}

impl<M: Clone> Topic<M> {
    pub fn new(name: Vec<u8>) -> Self {
        Self {
            name,
            sender: broadcast::channel(CAPACITY).0,
        }
    }

    pub fn with_capacity(name: Vec<u8>, capacity: usize) -> Self {
        Self {
            name,
            sender: broadcast::channel(capacity).0,
        }
    }

    pub fn name(&self) -> &[u8] {
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

impl<M: Clone> Hash for Topic<M> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl<M: Clone> Borrow<[u8]> for Topic<M> {
    fn borrow(&self) -> &[u8] {
        self.name.borrow()
    }
}
