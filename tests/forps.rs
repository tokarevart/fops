#[tokio::test]
async fn create_topic() {
    let topic_name = b"topic name";
    let topic = forps::topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn create_two_topics() {
    let topic_name = b"topic name";
    let _ = forps::topic(topic_name).await;
    let topic_name = b"another topic name";
    let topic = forps::topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn get_existing_topic() {
    let topic_name = b"topic name";
    let _ = forps::topic(topic_name).await;
    let topic = forps::topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn publish_and_receive_binary() {
    let topic_name = b"topic name";
    let topic = forps::topic(topic_name).await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    let msg = forps::BinaryMessage::from(b"message");
    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}

#[tokio::test]
async fn publish_and_receive_text() {
    let pubsub = forps::PubSub::new();

    let topic_name = b"topic name";
    let topic = pubsub.topic(topic_name).await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    let msg = forps::TextMessage::from("message");
    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}