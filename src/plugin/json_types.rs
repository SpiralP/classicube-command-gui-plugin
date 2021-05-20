use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonPlayer {
    pub id: u8,
    pub real_name: String,
    pub nick_name: String,
    pub group: String,
    pub rank: u8,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
#[serde(rename_all = "camelCase")]
pub enum JsonEvent {
    NewPlayers(Vec<JsonPlayer>),
    PlayerAdded(JsonPlayer),
    PlayerRemoved { id: u8 },
    PlayerChanged(JsonPlayer),
    WeDisconnected,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")] // {type: "NewAddress", data: "aa:aa:aa"}
#[serde(rename_all = "camelCase")]
pub enum JsonMessage {
    ChatCommand(String),
}
