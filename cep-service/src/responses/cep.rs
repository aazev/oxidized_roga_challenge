use serde::Deserialize;

use crate::structs::cep::Cep;

#[derive(Debug, Deserialize, Clone)]
pub struct CepResponse {
    pub cep: Cep,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub localidade: String,
    pub uf: String,
    pub ibge: String,
    pub gia: String,
    pub ddd: String,
    pub siafi: String,
}
