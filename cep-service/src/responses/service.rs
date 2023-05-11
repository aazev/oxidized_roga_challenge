use serde::Deserialize;

use super::{cep::CepResponse, error::CepError};

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum CepServiceResponse {
    CepNotFound(CepError),
    CepFound(CepResponse),
}
