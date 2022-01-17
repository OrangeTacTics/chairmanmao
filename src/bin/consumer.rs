use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::Message;

#[tokio::main]
async fn main() {
    let group_id = "foo";
    let brokers = "127.0.0.1:39537";
    let topics = vec!["foo"];

    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "smallest")
        .create()
        .unwrap();

    consumer
        .subscribe(&topics)
        .unwrap();

    loop {
        match consumer.recv().await {
            Err(e) => panic!("Kafka error: {}", e),
            Ok(m) => {
                let payload = m.payload().unwrap();
                let event = chairmanmao::events::Event::load(payload);
                if let Some(event) = event {
                    dbg!(event);
                } else {
                    println!("Can't parse");
                }
                consumer.commit_message(&m, CommitMode::Async).unwrap();
            }
        };
    }
}
