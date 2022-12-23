use crate::topic::Topic;
use std::collections::BTreeSet;
use tokio::sync::RwLock;

const TOPIC_CAPACITY: usize = 128;

pub struct PubSub {
    topics: RwLock<BTreeSet<Topic>>,
    topic_capacity: usize,
}

impl PubSub {
    pub const fn new() -> Self {
        Self {
            topics: RwLock::const_new(BTreeSet::new()),
            topic_capacity: TOPIC_CAPACITY,
        }
    }

    pub fn with_topic_capacity(capacity: usize) -> Self {
        Self {
            topics: RwLock::const_new(BTreeSet::new()),
            topic_capacity: capacity,
        }
    }

    pub async fn topic(&self, name: &str) -> Topic {
        self.topic_with_capacity(name, self.topic_capacity).await
    }

    pub async fn topic_with_capacity(&self, name: &str, capacity: usize) -> Topic {
        let topics = self.topics.read().await;
        if let Some(topic) = topics.get(name) {
            topic.clone()
        } else {
            let topic = Topic::new(name.to_string(), capacity);
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
