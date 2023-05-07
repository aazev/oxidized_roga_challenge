use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::responses::service::CepServiceResponse;

use super::cep::Cep;

#[derive(Debug, Clone)]
pub struct CepService {
    cache: HashMap<String, CepServiceResponse>,
}

impl CepService {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    async fn retrieve_cep(cep: &Cep) -> CepServiceResponse {
        let client = reqwest::Client::new();
        let url = format!("https://viacep.com.br/ws/{}/json/", cep);
        let response = client.get(&url).send().await.unwrap();
        let json = response.text().await.unwrap();
        let response = serde_json::from_str::<CepServiceResponse>(&json).unwrap();
        response
    }

    pub async fn get_address(&mut self, cep: Cep) -> CepServiceResponse {
        match self.cache.get(&cep.to_string()) {
            Some(response) => response.clone(),
            None => {
                let response = Self::retrieve_cep(&cep).await;
                self.cache.insert(cep.to_string(), response.clone());
                response
            }
        }
    }
}
