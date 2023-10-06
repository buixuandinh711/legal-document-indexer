use deadpool_postgres::Client;

use super::DbError;

pub struct CreateDocumentInfo {
    pub hash: String,
    pub onchain_division_id: String,
    pub onchain_officer_address: String,
    pub position_index: i16,
    pub signer_onchain_id: Vec<String>,
}

pub async fn create_document(
    client: &Client,
    position_info: &CreateDocumentInfo,
) -> Result<(), DbError> {
    let statement = include_str!("../sql/onchain_documents/insert_onchain_document.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|err| DbError::new("prepare insert_onchain_document", &err))?;

    let _ = client
        .execute(&statement, &[])
        .await
        .map_err(|err| DbError::new("execute insert_onchain_document", &err))?;

    Ok(())
}
