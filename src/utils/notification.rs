use reqwest::Client;
use std::env;

pub(crate) async fn send_notification(topic: String, message: String) -> () {
    let topic_url = env::var(topic).expect("Topic not found.");
    log::error!("{}", message);

    let _ = Client::new().post(topic_url).body(message).send().await;
}

pub(crate) fn spawn_notification(topic: String, message: String) -> () {
    tokio::spawn(async {
        send_notification(topic, message).await;
    });
}
