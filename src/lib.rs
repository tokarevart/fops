pub mod message;
pub mod pubsub;
pub mod subscription;
pub mod topic;

pub use message::Message;
pub use pubsub::PubSub;
pub use subscription::Subscription;
pub use topic::Topic;

static PUBSUB: PubSub = PubSub::new();

pub async fn topic(name: &str) -> Topic {
    PUBSUB.topic(name).await
}

pub async fn topic_with_capacity(name: &str, capacity: usize) -> Topic {
    PUBSUB.topic_with_capacity(name, capacity).await
}
