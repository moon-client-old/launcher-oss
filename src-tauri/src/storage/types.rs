use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LoginSettingData {
    pub uid: i64,
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameSettingData {
    pub memory: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WineSettingData {}
