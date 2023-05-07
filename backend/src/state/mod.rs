use std::sync::{Arc, RwLock};

use cep_service::structs::service::CepService;
use database::pool::connect;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub database_connection: MySqlPool,
    pub cep_service: Arc<RwLock<CepService>>,
}

impl ApplicationState {
    pub async fn new() -> Self {
        Self {
            database_connection: connect().await.unwrap(),
            cep_service: Arc::new(RwLock::new(CepService::new())),
        }
    }
}

// impl Deref for ApplicationState {
//     type Target = Arc<CepService>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.cep_service
//     }
// }
//
// impl DerefMut for ApplicationState {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.cep_service
//     }
// }
