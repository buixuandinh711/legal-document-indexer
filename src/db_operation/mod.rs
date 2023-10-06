pub mod onchain_divisions;
pub mod onchain_officers;
pub mod onchain_positions;
pub mod onchain_documents;

use derive_more::{Display, Error};
use std::fmt::Display;

#[derive(Default, Debug, Display, Error)]
pub struct DbError;

impl DbError {
    pub fn new(msg: &str, detail: &impl Display) -> Self {
        log::error!("DbError: {} -> {}", msg, detail);
        DbError {}
    }
}
