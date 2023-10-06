use crate::{
    contracts::legal_document_manager::PositionCreatedFilter,
    db_operation::onchain_positions::{create_position, CreatePositionInfo},
};
use deadpool_postgres::Pool;
use ethers::prelude::LogMeta;

pub async fn handle_position_created(db_pool: &Pool, event: PositionCreatedFilter, _meta: LogMeta) {
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let position_info = CreatePositionInfo {
                officer_address: event.officer_address.to_string(),
                division_onchain_id: event.division_id,
                position_index: event.position_index.as_usize() as i16,
                name: event.position_info.name,
                role: event.position_info.role,
            };
            let _ = create_position(&client, &position_info).await;
        }
        _ => {}
    }
}
