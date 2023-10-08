mod app_config;
mod contracts;
mod db_operation;
mod event_handler;
mod utils;

use crate::event_handler::{
    division_manager::handle_division_created, document_manager::handle_document_submitted,
    officer_manager::handle_officer_created, position_manager::handle_position_created,
};
use contracts::legal_document_manager::{self, LegalDocumentManagerEvents};
use deadpool_postgres::Pool;
use dotenv::dotenv;
use ethers::{
    core::types::Address,
    providers::{Http, Middleware, Provider},
};
use std::sync::Arc;
use tokio::time;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = app_config::AppConfig::from_env().unwrap();
    let db_pool = app_config.pg.create_pool(None, NoTls).unwrap();
    {
        let _ = db_pool.get().await.unwrap();
    } // panic if unable to connect
    log::info!("Database connected!");

    index_event(
        app_config.chain_rpc_url.clone(),
        app_config.legal_document_address.clone(),
        db_pool.clone(),
    )
    .await;

    Ok(())
}

pub async fn index_event(chain_rpc_url: String, legal_document_address: String, db_pool: Pool) {
    let provider = Provider::<Http>::try_from(chain_rpc_url).unwrap();
    let client = Arc::new(provider);
    let contract_address: Address = legal_document_address.parse().unwrap();
    let contract =
        legal_document_manager::LegalDocumentManager::new(contract_address, client.clone());

    tokio::fs::write("latest_block", 0.to_string().as_bytes())
        .await
        .unwrap();
    log::info!("Indexer started");

    loop {
        let latest_sync_block: u64 = tokio::fs::read_to_string("latest_block")
            .await
            .unwrap()
            .parse()
            .unwrap();
        let mut latest_block = client.get_block_number().await.unwrap().as_u64();

        if latest_block < latest_sync_block + 1 {
            continue;
        };

        if latest_block - latest_sync_block > 1000 {
            latest_block = latest_sync_block + 1000;
        }

        let events = contract
            .events()
            .from_block(latest_sync_block + 1)
            .to_block(latest_block)
            .query_with_meta()
            .await
            .unwrap();

        log::info!(
            "Read from {} to {}, event found: {}",
            latest_sync_block + 1,
            latest_block,
            events.len()
        );

        for e in events {
            match e {
                (LegalDocumentManagerEvents::OfficerCreatedFilter(event), meta) => {
                    handle_officer_created(&db_pool, event, meta).await;
                }
                (LegalDocumentManagerEvents::DivisionCreatedFilter(event), meta) => {
                    handle_division_created(&db_pool, event, meta).await;
                }
                (LegalDocumentManagerEvents::PositionCreatedFilter(event), meta) => {
                    handle_position_created(&db_pool, event, meta).await;
                }
                (LegalDocumentManagerEvents::DocumentSubmittedFilter(event), meta) => {
                    handle_document_submitted(&db_pool, event, meta, &client).await;
                }
                _ => {}
            }
        }

        tokio::fs::write("latest_block", (latest_block).to_string().as_bytes())
            .await
            .unwrap();

        time::sleep(time::Duration::from_secs(5)).await;
    }
}
