use maa_framework::controller::adb::MaaAdbControllerType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Debug, Clone, Copy)]
pub struct AppConfig {
    // Add your app-wise configurations here
    pub adb_controller_type: MaaAdbControllerType,
}
