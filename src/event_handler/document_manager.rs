use crate::{
    contracts::legal_document_manager::{self, DocumentPublishedFilter},
    db_operation::onchain_documents::{self, create_document, CreateDocumentInfo},
    utils,
};
use deadpool_postgres::Pool;
use ethers::prelude::LogMeta;

impl From<legal_document_manager::OfficerPosition> for onchain_documents::OfficerPosition {
    fn from(value: legal_document_manager::OfficerPosition) -> Self {
        onchain_documents::OfficerPosition {
            officer_address: utils::to_string_address(&value.officer_address),
            division_onchain_id: value.division_id,
            position_index: value.position_index.as_u32() as i16,
        }
    }
}

pub async fn handle_document_published(
    db_pool: &Pool,
    event: DocumentPublishedFilter,
    _meta: LogMeta,
) {
    let client = db_pool.get().await;
    match client {
        Ok(client) => {
            let document_info = CreateDocumentInfo {
                document_content_hash: utils::to_string_hash(&event.document_content_hash),
                number: event.document_info.number,
                name: event.document_info.name,
                division_id: event.document_info.division_id,
                published_timestamp: event.document_info.published_timestamp.as_u32() as i32,
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
