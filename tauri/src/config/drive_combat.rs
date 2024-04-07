use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
pub struct DriveCombatConfig {
    #[serde(default)]
    pub use_fuel: bool,
}
