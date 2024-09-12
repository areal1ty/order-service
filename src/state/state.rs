use redis::aio::MultiplexedConnection;
use redis::Client;
use sqlx::postgres::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_connection: Arc<Client>,
}

impl AppState {
    pub async fn new(db_url: &str, redis_url: &str) -> Arc<Self> {
        let db_pool = PgPool::connect(db_url)
            .await
            .expect("Failed to create Postgres connection pool");

        // Инициализация Redis клиента
        let redis_client = Client::open(redis_url)
            .expect("Invalid Redis URL");

        // Возвращаем состояние приложения в виде Arc
        Arc::new(AppState {
            db_pool,
            redis_connection: Arc::new(redis_client),
        })
    }

    pub async fn get_redis_connection(&self) -> redis::RedisResult<MultiplexedConnection> {
        let conn = self.redis_connection.get_multiplexed_async_connection().await?;
        Ok(conn)
    }
}