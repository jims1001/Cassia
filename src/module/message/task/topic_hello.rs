use rdkafka::message::BorrowedMessage;
use rdkafka::Message;
pub fn handle_topic_hello(msg: &BorrowedMessage) {

    println!("start handle topic hello");

    match msg.payload_view::<str>() {
        Some(Ok(payload)) => {
            println!("process topic-a: {}", payload);
        }
        Some(Err(e)) => {
            eprintln!("failed: {}", e);
        }
        None => {
            println!("payload is None");
        }
    }
}

