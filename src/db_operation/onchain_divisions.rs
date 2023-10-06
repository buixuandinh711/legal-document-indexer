use bytes::BytesMut;
use deadpool_postgres::Client;
use std::error::Error;
use tokio_postgres::types::{to_sql_checked, IsNull, ToSql, Type};

use super::DbError;

#[allow(unused)]
#[derive(Debug)]
pub enum DivisionStatus {
    NotCreated,
    Active,
    Deactivated,
}

impl ToSql for DivisionStatus {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match ty {
            &Type::INT2 => {
                let value: i16 = match self {
                    DivisionStatus::NotCreated => 0,
                    DivisionStatus::Active => 1,
                    DivisionStatus::Deactivated => 2,
                };
                value.to_sql(ty, out)
            }
            ty => Err(Box::new(DbError::new("DivisionStatus to SQL type", ty))),
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

pub struct CreateDivisionInfo {
    pub onchain_id: String,
    pub name: String,
    pub supervisory_id: String,
}

pub async fn create_division(
    client: &Client,
    division_info: &CreateDivisionInfo,
) -> Result<(), DbError> {
    let statement = include_str!("../sql/onchain_divisions/insert_onchain_division.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|err| DbError::new("prepare insert_division", &err))?;

    let _ = client
        .execute(
            &statement,
            &[
                &division_info.onchain_id,
                &division_info.name,
                &division_info.supervisory_id,
                &DivisionStatus::Active,
            ],
        )
        .await
        .map_err(|err| DbError::new("execute insert_division", &err))?;

    Ok(())
}
