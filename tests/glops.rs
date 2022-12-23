#[tokio::test]
async fn create_topic() {
    let topic_name = "topic name";
    let topic = glops::topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn create_two_topics() {
    let topic_name = "topic name";
    let _ = glops::topic(topic_name).await;
    let topic_name = "another topic name";
    let topic = glops::topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn get_existing_topic() {
    let topic_name = "topic name";
    let _ = glops::topic(topic_name).await;
    let topic = glops::topic(topic_name).await;
    assert_eq!(topic.name(), topic_name);
}

#[tokio::test]
async fn publish_and_receive_binary() {
    let topic_name = "topic name";
    let topic = glops::topic(topic_name).await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    let msg = glops::BinaryMessage::from(b"message");
    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}

#[tokio::test]
async fn publish_and_receive_text() {
    let pubsub = glops::PubSub::new();

    let topic_name = "topic name";
    let topic = pubsub.topic(topic_name).await;

    let mut subscription = topic.subscribe();
    let handle = tokio::spawn(async move { subscription.receive().await });

    let msg = glops::TextMessage::from("message");
    topic.publish(msg.clone()).unwrap();

    let recv_msg = handle.await.unwrap().unwrap();
    assert_eq!(recv_msg, msg);
}
