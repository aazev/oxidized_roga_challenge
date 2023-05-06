use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
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
