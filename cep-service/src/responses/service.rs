use serde::Deserialize;

use super::{cep::CepResponse, error::CepError};

#[derive(Debug, Deserialize, Clone)]
pub enum CepServiceResponse {
    CepNotFound(CepError),
    CepFound(CepResponse),
}
