use crate::{
    contracts::legal_document_manager::DivisionCreatedFilter,
    db_operation::onchain_divisions::{create_division, CreateDivisionInfo},
};
use deadpool_postgres::Pool;
use ethers::prelude::LogMeta;

pub async fn handle_division_created(db_pool: &Pool, event: DivisionCreatedFilter, _meta: LogMeta) {
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let division_info = CreateDivisionInfo {
                onchain_id: event.division_id,
                name: event.name,
                supervisory_id: event.supervisory_div_id,
            };
            let _ = create_division(&client, &division_info).await;
        }
        _ => {}
    }
}
