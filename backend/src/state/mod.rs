use cep_service::structs::service::CepService;
use database::pool::connect;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub database_connection: MySqlPool,
    pub cep_service: CepService,
}

impl ApplicationState {
    pub async fn new() -> Self {
        Self {
            database_connection: connect().await.unwrap(),
            cep_service: CepService::new(),
        }
    }
}
