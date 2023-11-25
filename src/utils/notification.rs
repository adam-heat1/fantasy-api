use reqwest::Client;

pub(crate)  async fn send_notification(topic: String, message: String) -> () {
    log::error!("{}", error_message);

    let _ =  Client::new()
        .post(topic)
        .body(message)
        .send()
        .await;
}

pub(crate) fn spawn_notification(topic: String, message: String) -> () {
    tokio::spawn(async {
        send_notification(topic, message).await;
    });
}