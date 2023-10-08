use crate::{
    contracts::legal_document_manager::OfficerCreatedFilter,
    db_operation::onchain_officers::{create_officer, CreateOfficerInfo},
    utils,
};
use deadpool_postgres::Pool;
use ethers::prelude::LogMeta;

pub async fn handle_officer_created(db_pool: &Pool, event: OfficerCreatedFilter, _meta: LogMeta) {
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let officer_info = CreateOfficerInfo {
                address: utils::to_string_address(&event.officer_address),
                name: event.info.name,
                date_of_birth: event.info.date_of_birth,
                sex: event.info.sex,
            };
            let _ = create_officer(&client, &officer_info).await;
        }
        _ => {}
    }
}
