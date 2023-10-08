use crate::{
    contracts::legal_document_manager::DocumentSubmittedFilter,
    db_operation::onchain_documents::{create_document, CreateDocumentInfo},
    utils,
};
use deadpool_postgres::Pool;
use ethers::{
    prelude::LogMeta,
    providers::{JsonRpcClient, Middleware, Provider},
};

pub async fn handle_document_submitted<T: JsonRpcClient>(
    db_pool: &Pool,
    event: DocumentSubmittedFilter,
    meta: LogMeta,
    provider: &Provider<T>,
) {
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let tx_hash = meta.transaction_hash;
            let tx = provider.get_transaction(tx_hash).await;
            if tx.is_err() {
                log::error!("Handler: handle_document_submitted get tx");
                return;
            }
            let tx = tx.unwrap();
            if tx.is_none() {
                log::error!("Handler: handle_document_submitted tx not found");
                return;
            }
            let tx = tx.unwrap();
            let submitter = tx.from;
            let document_info = CreateDocumentInfo {
                hash: utils::to_string_hash(&event.document_hash),
                onchain_division_id: event.division_id,
                submitter_address: utils::to_string_address(&submitter),
                position_index: event.position_index.as_usize() as i16,
                signers_address: event
                    .signers
                    .iter()
                    .map(|s| utils::to_string_address(s))
                    .collect(),
            };
            let _ = create_document(&client, &document_info).await;
        }
        _ => {}
    }
}
