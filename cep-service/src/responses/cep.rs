use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CepResponse {
    // pub cep: String,
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
