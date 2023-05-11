use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use reqwest::header::{self, HeaderMap};

use crate::responses::service::CepServiceResponse;

use super::cep::Cep;

#[derive(Debug, Clone)]
pub struct CepService {
    cache: Arc<RwLock<HashMap<String, CepServiceResponse>>>,
}

impl CepService {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn retrieve_cep(cep: &Cep) -> CepServiceResponse {
        let mut headers = HeaderMap::new();
        headers.insert(header::ACCEPT, "application/json".parse().unwrap());
        let client = reqwest::Client::new();
        let url = format!("https://viacep.com.br/ws/{}/json/", cep);
        let response = client.get(&url).headers(headers).send().await.unwrap();
        let json = response.text().await.unwrap();
        let result = serde_json::from_str::<CepServiceResponse>(&json).unwrap();
        result
    }

    pub async fn get_address(&self, cep: Cep) -> CepServiceResponse {
        // Try to read from cache first
        {
            let cache_read = self.cache.read().unwrap();
            if let Some(response) = cache_read.get(&cep.to_string()) {
                return response.clone();
            }
        }

        // If not found in cache, retrieve and store the response
        let response = Self::retrieve_cep(&cep).await;
        {
            let mut cache_write = self.cache.write().unwrap();
            cache_write.insert(cep.to_string(), response.clone());
        }
        response
    }
}
