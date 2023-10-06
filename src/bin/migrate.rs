use deadpool_postgres::{Client, PoolError};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[derive(Debug, serde::Deserialize)]
struct PGConfig {
    pg: deadpool_postgres::Config,
}

async fn create_tables(client: &Client) -> Result<(), PoolError> {
    let statement = include_str!("../sql/create_table.sql");
    client.batch_execute(statement).await?;
    println!("Table created");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config_ = config::Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: PGConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();
    let client = pool.get().await.unwrap();
    println!("Connect successfully");

    create_tables(&client).await.unwrap();
}
