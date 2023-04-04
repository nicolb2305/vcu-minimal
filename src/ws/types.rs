use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};

use crate::api::types::LolChampSelectChampSelectSession;

#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub enum SubscriptionType {
    #[default]
    All,
    LolChampSelectV1Session,
}

impl Display for SubscriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::All => "OnJsonApiEvent",
            Self::LolChampSelectV1Session => "OnJsonApiEvent_lol-champ-select_v1_session",
        };
        write!(f, "{text}")
    }
}

#[derive(Deserialize_tuple, Serialize_tuple, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketEvent {
    pub op_code: OpCode,
    pub event: String,
    pub data: WebSocketResponse,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum OpCode {
    Subscribe = 5,
    Unsubscribe = 6,
    #[default]
    Update = 8,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum EventType {
    Create,
    #[default]
    Update,
    Delete,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketData<T> {
    pub data: T,
    pub event_type: EventType,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
#[non_exhaustive]
pub enum WebSocketResponse {
    ChampSelect(WebSocketData<LolChampSelectChampSelectSession>),
}
