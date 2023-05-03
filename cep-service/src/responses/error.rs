use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CepError {
    pub erro: bool,
    pub mensagem: Option<String>,
}
