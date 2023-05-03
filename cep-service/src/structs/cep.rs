use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cep {
    region: u8,
    subregion: u8,
    sector: u8,
    subsector: u8,
    division: u8,
    distribution: u16,
}

impl Cep {
    pub fn new(cep: String) -> Result<Self, Box<dyn Error>> {
        let cep = cep.replace("-", "");
        let region = cep[0..1].parse::<u8>().unwrap();
        let subregion = cep[1..2].parse::<u8>().unwrap();
        let sector = cep[2..3].parse::<u8>().unwrap();
        let subsector = cep[3..4].parse::<u8>().unwrap();
        let division = cep[4..5].parse::<u8>().unwrap();
        let distribution = cep[5..8].parse::<u16>().unwrap();
        match distribution {
            1..=999 => Ok(Self {
                region,
                subregion,
                sector,
                subsector,
                division,
                distribution,
            }),
            _ => Err("Invalid CEP".into()),
        }
    }
}

impl TryFrom<String> for Cep {
    type Error = Box<dyn Error>;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Display for Cep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}-{}",
            self.region,
            self.subregion,
            self.sector,
            self.subsector,
            self.division,
            self.distribution
        )
    }
}
