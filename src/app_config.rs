use serde::Deserialize;
#[derive(Debug, Default, Deserialize)]
pub struct AppConfig {
    pub chain_rpc_url: String,
    pub legal_document_address: String,
    pub pg: deadpool_postgres::Config,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
