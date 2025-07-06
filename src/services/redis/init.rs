use deadpool_redis::{Config, Pool, Runtime};
use once_cell::sync::Lazy;
const REDIS_URL: &str = "redis://:password@127.0.0.1:7001/";
pub static REDIS_POOL: Lazy<Pool> = Lazy::new(|| {
    let cfg = Config::from_url(REDIS_URL);
    cfg.create_pool(Some(Runtime::Tokio1)).expect(" Redis connection pool creation error")
});