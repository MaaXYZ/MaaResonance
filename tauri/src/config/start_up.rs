use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum ClientType {
    #[default]
    Official,
    Bilibili,
}

impl ClientType {
    pub fn get_package_name(self) -> String {
        match self {
            ClientType::Official => "com.hypergryph.arknights/com.u8.sdk.U8UnityContext".to_owned(),
            ClientType::Bilibili => "com.hypergryph.arknights.bilibili".to_owned(),
        }
    }
}

impl From<String> for ClientType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "bilibili" => ClientType::Bilibili,
            _ => ClientType::Official,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct StartUpConfig {
    #[serde(default)]
    pub client_type: ClientType,
}
