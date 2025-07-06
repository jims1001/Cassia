use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use once_cell::sync::Lazy;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use crate::services::kafka::KafkaDispatcher;
pub static KAFKA_DISPATCHER: Lazy<KafkaDispatcher> = Lazy::new(|| KafkaDispatcher::new());


pub static PRODUCER: Lazy<FutureProducer> = Lazy::new(|| {
    ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Kafka Producer init failed")
});

pub async fn send_kafka_message(topic: &str, key: &str, payload: &str) {
    let record = FutureRecord::to(topic)
        .payload(payload)
        .key(key);

    match PRODUCER.send(record, Duration::from_secs(1)).await {
        Ok(delivery) => println!("✅ Kafka send message success: {:?}", delivery),
        Err((err, _)) => eprintln!("❌ Kafka send message failed: {:?}", err),
    }
}


pub async fn start_kafka_consumer(brokers: &str, group_id: &str, topic: &str) {


    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("create Kafka topic failed");

    consumer
        .subscribe(&[topic])
        .expect("subscribe Kafka topic failed.");

    println!("[Kafka] consumer，listen topic: {}", topic);

    tokio::spawn(async move {
        loop {
            match consumer.recv().await {
                Ok(msg) => {
                    match msg.payload_view::<str>() {
                        Some(Ok(payload)) => {
                            let topic = msg.topic();
                            println!("[Kafka:{}] receive message: {}", topic, payload);
                            // 使用 Dispatcher 分发处理
                            KAFKA_DISPATCHER.dispatch(&msg);
                        },
                        Some(Err(e)) => eprintln!("Kafka message decode failed : {}", e),
                        None => println!("Kafka message payload is None"),
                    }
                }
                Err(e) => {
                    eprintln!("Kafka failed: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    });
}
