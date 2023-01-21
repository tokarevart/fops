use crate::topic::{self, Topic};
use std::collections::BTreeSet;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct PubSub<M: Clone> {
    topics: RwLock<BTreeSet<Topic<M>>>,
    topic_capacity: usize,
}

impl<M: Clone> PubSub<M> {
    pub const fn new() -> Self {
        Self {
            topics: RwLock::const_new(BTreeSet::new()),
            topic_capacity: topic::CAPACITY,
        }
    }

    pub fn with_topic_capacity(capacity: usize) -> Self {
        Self {
            topics: RwLock::const_new(BTreeSet::new()),
            topic_capacity: capacity,
        }
    }

    pub async fn topic(&self, name: &[u8]) -> Topic<M> {
        self.topic_with_capacity(name, self.topic_capacity).await
    }

    pub async fn topic_with_capacity(&self, name: &[u8], capacity: usize) -> Topic<M> {
        let topics = self.topics.read().await;
        if let Some(topic) = topics.get(name) {
            topic.clone()
        } else {
            let topic = Topic::with_capacity(name.to_vec(), capacity);
            drop(topics);
            let mut topics = self.topics.write().await;
            if !topics.insert(topic.clone()) {
                topics.get(name).unwrap().clone()
            } else {
                topic
            }
        }
    }
}
