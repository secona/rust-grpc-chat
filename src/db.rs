use std::sync::OnceLock;
use sqlx::postgres::PgPool;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn init_db_pool() -> Result<(), sqlx::Error> {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/grpc_chat").await?;
    DB_POOL.set(pool).expect("DB_POOL already initialized");
    Ok(())
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("DB_POOL is not initialized")
}
