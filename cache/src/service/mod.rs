use std::env;

use cep_service::{
    responses::{cep::CepResponse, service::CepServiceResponse},
    structs::cep::Cep,
};
use dotenv::dotenv;
use redis::cluster::ClusterClient;

pub struct CachedCepServer {
    client: ClusterClient,
}

impl CachedCepServer {
    pub fn new() -> Self {
        dotenv().ok();
        let cluster = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set")
            .split(',')
            .map(|a| a.to_string())
            .collect();
        Self {
            client: ClusterClient::new(cluster).unwrap(),
        }
    }

    pub fn get_cep(cep: Cep) -> CepServiceResponse {}
}
