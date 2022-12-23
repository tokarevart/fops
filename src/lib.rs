pub mod message;
pub mod pubsub;
pub mod subscription;
pub mod topic;

pub use message::binary_message::BinaryMessage;
pub use message::text_message::TextMessage;
pub use pubsub::PubSub;
pub use subscription::Subscription;
pub use topic::Topic;

static PUBSUB: PubSub<BinaryMessage> = PubSub::new();

pub async fn topic(name: &str) -> Topic<BinaryMessage> {
    PUBSUB.topic(name).await
}

pub async fn topic_with_capacity(name: &str, capacity: usize) -> Topic<BinaryMessage> {
    PUBSUB.topic_with_capacity(name, capacity).await
}
