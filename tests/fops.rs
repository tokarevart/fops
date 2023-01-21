use fops::PubSub;

#[tokio::test]
async fn create_topic() {
    let pubsub = PubSub::<Vec<u8>>::new();

    let topic_name = b"topic name";
    let topic = pubsub.topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn create_two_topics() {
    let pubsub = PubSub::<Vec<u8>>::new();

    let topic_name = b"topic name";
    let _ = pubsub.topic(topic_name).await;
    let topic_name = b"another topic name";
    let topic = pubsub.topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn get_existing_topic() {
    let pubsub = PubSub::<Vec<u8>>::new();

    let topic_name = b"topic name";
    let _ = pubsub.topic(topic_name).await;
    let topic = pubsub.topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn publish_and_receive_binary() {
    let pubsub = PubSub::new();

    let topic_name = b"topic name";
    let topic = pubsub.topic(topic_name).await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    let msg = b"message".to_vec();
    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}

#[tokio::test]
async fn publish_and_receive_text() {
    let pubsub = PubSub::new();

    let topic_name = b"topic name";
    let topic = pubsub.topic(topic_name).await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    let msg = "message".to_string();
    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}
