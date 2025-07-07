use std::sync::Arc;
use actix_web::{get, web, App, HttpServer, Responder};
pub mod extend;
use Cassia::services::redis::REDIS_POOL;
use extend::services::postgres::{init_pg_pool, PG_POOL};
use extend::services::kafka::{start_kafka_consumer};
use extend::module::task;
use deadpool_redis::redis::AsyncCommands;
use sqlx::Row;
use rdkafka::message::BorrowedMessage;
use Cassia::services::kafka::{KafkaHandler, KAFKA_DISPATCHER};
use Cassia::services::nacos::GLOBAL_CONFIG;
use extend::services::nacos::init_config_watcher;
use extend::services::grpc::start_grpc_server;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello world !".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    register_handlers().await;

    let mut conn = REDIS_POOL.get().await.expect("get Redis connection");
    let _: () = conn.set("key", "hello rust").await.unwrap();

    let val: String = conn.get("key").await.unwrap();
    println!(" value from redis：{}", val);

    init_pg_pool().await;

    init_config_watcher().await;

    start_grpc_server().await.expect("TODO: panic message");

    let config = GLOBAL_CONFIG.read().unwrap();
    // println!("🚀 Starting server on port {port}");

    // 获取连接池
    let pool = PG_POOL.get().expect("连接池尚未初始化");

    // 测试查询
    let row = sqlx::query("SELECT now()::timestamp")
        .fetch_one(pool)
        .await
        .expect("search failed");

    let now: chrono::NaiveDateTime = row.get(0);
    println!("current date: {}", now);

    // start Kafka  consumer task（background running）
    start_kafka_consumer("localhost:9092", "my-group", "test-topic").await;
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("starting server");

    HttpServer::new(|| {
        App::new().service(greet).service(index).route("/ws/", web::get().to(extend::services::ws::ws_index))
    })
        .bind(("127.0.0.1", config.port))?
        .run()
        .await
}

async  fn register_handlers() {
    KAFKA_DISPATCHER.register_handler(
        "test-topic",
        Arc::new(|msg: &BorrowedMessage| {
            println!("{:?}", msg);
            task::topic_hello::handle_topic_hello(msg);
            }) as KafkaHandler
    );

    KAFKA_DISPATCHER.register_handler(
        "topic-b",
        Arc::new(|msg: &BorrowedMessage| {
            task::topic_hello::handle_topic_hello(msg);
        }) as KafkaHandler
    );
}