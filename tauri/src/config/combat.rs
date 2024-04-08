use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct CombatConfig {
    #[serde(default)]
    pub times: u32,
}
