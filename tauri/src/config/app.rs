use maa_framework::controller::adb::MaaAdbControllerType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone)]
pub struct AppConfig {
    // Add your app-wise configurations here
    #[serde(default)]
    pub adb_controller_type: MaaAdbControllerType,
}
