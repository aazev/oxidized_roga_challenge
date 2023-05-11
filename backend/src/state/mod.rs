use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use cep_service::structs::service::CepService;
use database::{models::user::UserModel, pool::connect};
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub database_connection: MySqlPool,
    user_cache: Arc<RwLock<HashMap<String, UserModel>>>,
    pub cep_service: CepService,
}

impl ApplicationState {
    pub async fn new() -> Self {
        Self {
            database_connection: connect().await.unwrap(),
            user_cache: Arc::new(RwLock::new(HashMap::new())),
            cep_service: CepService::new(),
        }
    }
}

impl ApplicationState {
    pub fn user_cached(&self, token: &str) -> bool {
        let cache = self.user_cache.read().unwrap();
        cache.get(token).is_some()
    }

    pub fn get_user_cache(&self, token: &str) -> Option<UserModel> {
        let cache = self.user_cache.read().unwrap();
        cache.get(token).cloned()
    }

    pub fn insert_user_cache(&self, token: &str, user: &UserModel) {
        let mut cache = self.user_cache.write().unwrap();
        cache.insert(token.to_string(), user.clone());
    }
}
