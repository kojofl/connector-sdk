use serde::{ser::SerializeMap, Deserialize, Serialize};

use super::ConnectorRequestContent;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CanCreateOutgoingRequestRequest {
    pub content: ConnectorRequestContent,
    pub peer: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOutgoingRequestRequest {
    pub content: ConnectorRequestContent,
    pub peer: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DecideRequest {
    pub items: Vec<DecideRequestContent>,
}

impl FromIterator<DecideRequestContent> for DecideRequest {
    fn from_iter<T: IntoIterator<Item = DecideRequestContent>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DecideRequestContent {
    Item(DecideRequestItem),
    Group(DecideRequestItemGroup),
}

impl From<DecideRequestItem> for DecideRequestContent {
    fn from(value: DecideRequestItem) -> Self {
        Self::Item(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DecideRequestItemGroup {
    pub items: Vec<DecideRequestItem>,
}

impl FromIterator<DecideRequestItem> for DecideRequestItemGroup {
    fn from_iter<T: IntoIterator<Item = DecideRequestItem>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

// TODO: implement Deserialize
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum DecideRequestItem {
    Accept,
    Reject {
        code: Option<String>,
        message: Option<String>,
    },
}

impl DecideRequestItem {
    pub fn accept() -> DecideRequestItem {
        Self::Accept
    }

    pub fn reject() -> DecideRequestItem {
        Self::Reject {
            code: None,
            message: None,
        }
    }
}

impl Serialize for DecideRequestItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DecideRequestItem::Accept => {
                let mut state = serializer.serialize_map(Some(1))?;
                state.serialize_entry("accept", &true)?;
                state.end()
            }
            DecideRequestItem::Reject { code, message } => {
                let len = { 1 + code.is_some() as usize + message.is_some() as usize };
                let mut state = serializer.serialize_map(Some(len))?;
                state.serialize_entry("accept", &false)?;
                if let Some(c) = code.as_ref() {
                    state.serialize_entry("code", c)?;
                }
                if let Some(m) = message.as_ref() {
                    state.serialize_entry("message", m)?;
                }
                state.end()
            }
        }
    }
}
