use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use once_cell::sync::OnceCell;
pub static PG_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

const DATABASE_URL: &str = "postgres://root:pwd124@localhost:5432/sc_agent_chat";

pub async fn init_pg_pool() {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await
        .expect("connection PostgresSQL database connection failed");

    PG_POOL.set(pool).expect("connection PostgresSQL pool was already initialized");
}
