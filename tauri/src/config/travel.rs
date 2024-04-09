use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct TravelConfig {
    #[serde(default)]
    pub destination: String,
}
