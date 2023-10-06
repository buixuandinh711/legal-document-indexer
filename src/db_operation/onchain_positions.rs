use bytes::BytesMut;
use deadpool_postgres::Client;
use std::error::Error;
use tokio_postgres::types::{to_sql_checked, IsNull, ToSql, Type};

use super::DbError;

#[derive(Debug)]
pub enum PositionRole {
    Revoked,
    DivisionAdmin,
    Manager,
    Staff,
}

impl ToSql for PositionRole {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        match ty {
            &Type::INT2 => {
                let value: i16 = match self {
                    PositionRole::Revoked => 0,
                    PositionRole::DivisionAdmin => 1,
                    PositionRole::Manager => 2,
                    PositionRole::Staff => 3,
                };
                value.to_sql(ty, out)
            }
            ty => Err(Box::new(DbError::new("PositionRole to SQL type", ty))),
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

impl TryFrom<&u8> for PositionRole {
    type Error = DbError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PositionRole::Revoked),
            1 => Ok(PositionRole::DivisionAdmin),
            2 => Ok(PositionRole::Manager),
            3 => Ok(PositionRole::Staff),
            val => Err(DbError::new("convert from u8 to PositionRole", val)),
        }
    }
}

pub struct CreatePositionInfo {
    pub officer_address: String,
    pub division_onchain_id: String,
    pub position_index: i16,
    pub name: String,
    pub role: u8,
}

pub async fn create_position(
    client: &Client,
    position_info: &CreatePositionInfo,
) -> Result<(), DbError> {
    let statement = include_str!("../sql/onchain_positions/insert_onchain_position.sql");
    let statement = client
        .prepare(&statement)
        .await
        .map_err(|err| DbError::new("prepare insert_onchain_position", &err))?;

    let _ = client
        .execute(
            &statement,
            &[
                &position_info.officer_address,
                &position_info.division_onchain_id,
                &position_info.position_index,
                &position_info.name,
                &PositionRole::try_from(&position_info.role)?,
            ],
        )
        .await
        .map_err(|err| DbError::new("execute insert_onchain_position", &err))?;

    Ok(())
}
