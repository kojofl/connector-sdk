use serde::{Deserialize, Serialize};

pub mod account;
pub mod attributes;
pub mod challenges;
pub mod connector_request;
pub mod files;
pub mod messages;
pub mod monitoring;
pub mod relationship;
pub mod relationship_templates;
pub mod tokens;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShareInfo {
    request_reference: String,
    peer: String,
    source_attribute: Option<String>,
}
