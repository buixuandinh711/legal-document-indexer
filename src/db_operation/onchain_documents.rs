use deadpool_postgres::Client;
use tokio_postgres::types::ToSql;

use super::DbError;

pub struct OfficerPosition {
    pub officer_address: String,
    pub division_onchain_id: String,
    pub position_index: i16,
}

pub struct CreateDocumentInfo {
    pub document_content_hash: String,
    pub number: String,
    pub name: String,
    pub doc_type: String,
    pub division_id: String,
    pub published_timestamp: i32,
    pub publisher: OfficerPosition,
    pub signers: Vec<OfficerPosition>,
}

pub async fn create_document(
    client: &Client,
    document_info: &CreateDocumentInfo,
) -> Result<(), DbError> {
    let statement = include_str!("../sql/onchain_documents/insert_onchain_document.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|err| DbError::new("prepare insert_onchain_document", &err))?;

    let _ = client
        .execute(
            &statement,
            &[
                &document_info.document_content_hash,
                &document_info.number,
                &document_info.name,
                &document_info.doc_type,
                &document_info.division_id,
                &document_info.published_timestamp,
                &document_info.publisher.officer_address,
                &document_info.publisher.division_onchain_id,
                &document_info.publisher.position_index,
            ],
        )
        .await
        .map_err(|err| DbError::new("execute insert_onchain_document", &err))?;

    if !document_info.signers.is_empty() {
        insert_signers(
            client,
            &document_info.document_content_hash,
            &document_info.signers,
        )
        .await?;
    }

    Ok(())
}

async fn insert_signers(
    client: &Client,
    document_content_hash: &str,
    signers: &[OfficerPosition],
) -> Result<(), DbError> {
    let mut statement =
        include_str!("../sql/onchain_documents/insert_document_signatures.sql").to_owned();
    let mut params = Vec::<&(dyn ToSql + Sync)>::new();
    let mut i = 1;

    for column in signers.iter() {
        if i == 1 {
            statement = format!("{} (${},${},${},${})", statement, i, i + 1, i + 2, i + 3);
        } else {
            statement = format!("{}, (${},${},${},${})", statement, i, i + 1, i + 2, i + 3);
        }
        params.push(&document_content_hash);
        params.push(&column.officer_address);
        params.push(&column.division_onchain_id);
        params.push(&column.position_index);
        i = i + 4;
    }

    statement += " ON CONFLICT (document_content_hash, signers_address, division_onchain_id, position_index) DO NOTHING;";

    let statement = client
        .prepare(&statement)
        .await
        .map_err(|err| DbError::new("prepare insert_document_signatures", &err))?;

    let _ = client
        .execute(&statement, &params[..])
        .await
        .map_err(|err| DbError::new("execute insert_document_signatures", &err))?;

    Ok(())
}
