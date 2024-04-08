use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct DriveCombatConfig {
    #[serde(default)]
    pub use_fuel: bool,
}
