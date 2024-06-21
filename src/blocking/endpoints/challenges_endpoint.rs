use reqwest::Method;

use crate::{
    blocking::ConnectorClient,
    types::challenges::{
        ConnectorChallenge, ConnectorChallengeValidationResult, ValidateChallengeRequest,
    },
};

/// All endpoints related to creating and verifying challenges.
impl<'a> ConnectorClient<'a> {
    pub fn create_device_challenge(
        &self,
    ) -> Result<ConnectorChallenge, crate::connector_errors::Error> {
        self.request(
            "api/v2/challenges",
            Method::POST,
            Some(r#"{ "challengeType": "Device" }"#.to_owned()),
        )
    }

    pub fn create_identity_challenge(
        &self,
    ) -> Result<ConnectorChallenge, crate::connector_errors::Error> {
        self.request(
            "api/v2/challenges",
            Method::POST,
            Some(r#"{ "challengeType": "Identity" }"#.to_owned()),
        )
    }

    pub fn create_relationship_challenge(
        &self,
        rel: &str,
    ) -> Result<ConnectorChallenge, crate::connector_errors::Error> {
        self.request(
            "api/v2/challenges",
            Method::POST,
            Some(format!(
                r#"{{ "challengeType": "Device", "relationship": {rel} }}"#
            )),
        )
    }

    pub fn validate_challenge(
        &self,
        req: &ValidateChallengeRequest<'_>,
    ) -> Result<
        ConnectorChallengeValidationResult<serde_json::Value, serde_json::Value>,
        crate::connector_errors::Error,
    > {
        self.request(
            "api/v2/Challenges/Validate",
            Method::POST,
            Some(serde_json::to_string(req).unwrap()),
        )
    }
}
