pub use super::seed::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallFile {
    pub seed: SeedConf,
}

impl Validate for InstallFile {
    fn validate(&mut self) -> Result<(), ValidationError> {
        self.seed.validate()
    }
}
