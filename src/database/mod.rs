use sqlx::{Pool, Postgres, migrate::Migrator};
use anyhow::Result;

pub type Database = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<Database> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

pub async fn migrate(pool: &Database) -> Result<()> {
    let migrator = Migrator::new(std::path::Path::new("./src/database/migrations")).await?;
    migrator.run(pool).await?;
    Ok(())
}