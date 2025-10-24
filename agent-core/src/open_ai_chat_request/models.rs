use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllowedMcpToolCallsJsonModel {
    #[serde(rename = "type")]
    pub r#type: String,
    pub server_label: String,
    pub server_url: String,
    pub server_description: String,
    pub allowed_tools: Vec<String>,
    pub require_approval: String,
}

pub enum HistoryItemRole {
    User,
    Assistant,
}

impl HistoryItemRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            HistoryItemRole::User => "user",
            HistoryItemRole::Assistant => "assistant",
        }
    }
}

pub struct HistoryItem {
    pub role: HistoryItemRole,
    pub text: String,
}
