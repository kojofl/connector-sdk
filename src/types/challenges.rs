use serde::{Deserialize, Serialize};

use super::relationship::responses::ConnectorRelationship;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorChallenge {
    pub id: String,
    pub expires_at: String,
    pub created_by: Option<String>,
    pub created_by_device: Option<String>,
    #[serde(rename = "type")]
    pub _type: String,
    pub signature: String,
    pub challenge_string: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectorChallengeValidationResult<T, C> {
    is_valid: bool,
    corresponding_relationship: Option<ConnectorRelationship<T, C>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValidateChallengeRequest<'a> {
    challenge_string: &'a str,
    signature: &'a str,
}
