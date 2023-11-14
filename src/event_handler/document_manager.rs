use std::time::UNIX_EPOCH;

use crate::{
    contracts::legal_document_manager::{self, DocumentPublishedFilter},
    db_operation::onchain_documents::{self, create_document, CreateDocumentInfo},
    utils,
};
use deadpool_postgres::Pool;
use ethers::{
    prelude::{
        providers::{JsonRpcClient, Provider},
        LogMeta,
    },
    providers::Middleware,
};

impl From<legal_document_manager::OfficerPosition> for onchain_documents::OfficerPosition {
    fn from(value: legal_document_manager::OfficerPosition) -> Self {
        onchain_documents::OfficerPosition {
            officer_address: utils::to_string_address(&value.officer_address),
            division_onchain_id: value.division_id,
            position_index: value.position_index.as_u32() as i16,
        }
    }
}

pub async fn handle_document_published<T: JsonRpcClient>(
    db_pool: &Pool,
    event: DocumentPublishedFilter,
    meta: LogMeta,
    provider: &Provider<T>,
) {
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let block_hash = meta.block_hash;
            let block = provider.get_block(block_hash).await;
            if block.is_err() {
                log::error!("Handler: handle_document_submitted get block");
                return;
            }
            let block = block.unwrap();
            if block.is_none() {
                log::error!("Handler: handle_document_submitted block not found");
                return;
            }
            let block = block.unwrap();
            let block_timestamp = block.timestamp;
            let duration = std::time::Duration::from_secs(block_timestamp.as_u64());
            let published_timestamp = UNIX_EPOCH + duration;

            let document_info = CreateDocumentInfo {
                document_content_hash: utils::to_string_hash(&event.document_content_hash),
                number: event.document_info.number,
                name: event.document_info.name,
                doc_type: event.document_info.doc_type,
                division_id: event.document_info.division_id,
                published_timestamp,
                publisher: event.publisher.into(),
                signers: event
                    .signers
                    .into_iter()
                    .map(|signer| signer.into())
                    .collect(),
            };
            let _ = create_document(&client, &document_info).await;
        }
        _ => {}
    }
}
