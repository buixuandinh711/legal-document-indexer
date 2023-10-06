use bytes::BytesMut;
use deadpool_postgres::Client;
use std::error::Error;
use tokio_postgres::types::{to_sql_checked, IsNull, ToSql, Type};

use super::DbError;

#[allow(unused)]
#[derive(PartialEq, Debug)]
pub enum OfficerStatus {
    NotCreated,
    Active,
    Deactivated,
}

impl ToSql for OfficerStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match ty {
            &Type::INT2 => {
                let value: i16 = match self {
                    OfficerStatus::NotCreated => 0,
                    OfficerStatus::Active => 1,
                    OfficerStatus::Deactivated => 2,
                };
                value.to_sql(ty, out)
            }
            t => Err(Box::new(DbError::new("OfficerStatus to SQL type", t))),
        }
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        *ty == Type::INT2
    }

    to_sql_checked!();
}

pub struct CreateOfficerInfo {
    pub address: String,
    pub name: String,
    pub date_of_birth: String,
    pub sex: String,
}

pub async fn create_officer(
    client: &Client,
    officer_info: &CreateOfficerInfo,
) -> Result<(), DbError> {
    let statement = include_str!("../sql/onchain_officers/insert_onchain_officer.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|err| DbError::new("prepare insert_onchain_officer", &err))?;

    let _ = client
        .execute(
            &statement,
            &[
                &officer_info.address,
                &officer_info.name,
                &officer_info.date_of_birth,
                &officer_info.sex,
                &OfficerStatus::Active,
            ],
        )
        .await
        .map_err(|err| DbError::new("execute insert_onchain_officer", &err))?;

    Ok(())
}
