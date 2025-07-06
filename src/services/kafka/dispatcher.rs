// kafka_dispatcher.rs
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;

pub type KafkaHandler = Arc<dyn Fn(&BorrowedMessage) + Send + Sync + 'static>;

pub struct KafkaDispatcher {
    handlers: RwLock<HashMap<String, KafkaHandler>>,
}

impl KafkaDispatcher {
    pub fn new() -> Self {
        KafkaDispatcher {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_handler(&self, topic: &str, handler: KafkaHandler) {
        self.handlers
            .write()
            .unwrap()
            .insert(topic.to_string(), handler);
    }

    pub fn dispatch(&self, msg: &BorrowedMessage) {
        let topic = msg.topic();
        if let Some(handler) = self.handlers.read().unwrap().get(topic) {
            handler(msg);
        } else {
            eprintln!("not found topic [{}]  handler", topic);
        }
    }

    pub fn all_topics(&self) -> Vec<String> {
        self.handlers
            .read()
            .unwrap()
            .keys()
            .cloned()
            .collect()
    }
}
