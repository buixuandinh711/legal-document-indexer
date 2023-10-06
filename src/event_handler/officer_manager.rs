use crate::{
    contracts::legal_document_manager::OfficerCreatedFilter,
    db_operation::onchain_officers::{create_officer, CreateOfficerInfo},
};
use deadpool_postgres::Pool;
use ethers::prelude::LogMeta;

pub async fn handle_officer_created(db_pool: &Pool, event: OfficerCreatedFilter, _meta: LogMeta) {
    log::info!("{}", event.officer_address.to_string());
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let officer_info = CreateOfficerInfo {
                address: event.officer_address.to_string(),
                name: event.info.name,
                date_of_birth: event.info.date_of_birth,
                sex: event.info.sex,
            };
            let _ = create_officer(&client, &officer_info).await;
        }
        _ => {}
    }
}
