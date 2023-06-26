use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LoginSettingData {
    pub uid: i64,
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct GameSettingData {
    pub memory: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct WineSettingData {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionSettingData {
    pub selections: Vec<VersionSelectionData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionSelectionData {
    pub channel: String,
    pub preferred_version: String,
    pub requires_latest: bool,
}
