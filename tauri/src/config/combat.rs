use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
pub struct CombatConfig {
    #[serde(default = "default_combat_times")]
    pub times: u32,
}

fn default_combat_times() -> u32 {
    u32::MAX
}