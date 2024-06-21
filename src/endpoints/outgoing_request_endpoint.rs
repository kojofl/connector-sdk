use reqwest::Method;

use crate::{
    types::connector_request::{
        get_requests::GetRequestsRequest,
        requests::{CanCreateOutgoingRequestRequest, CreateOutgoingRequestRequest},
        ConnectorRequest, ConnectorRequestValidationResult,
    },
    ConnectorClient,
};

/// All endpoints related to creating and managing outgoing requests.
impl<'a> ConnectorClient<'a> {
    pub async fn can_create_request(
        &self,
        req: &CanCreateOutgoingRequestRequest,
    ) -> Result<ConnectorRequestValidationResult, crate::connector_errors::Error> {
        self.request(
            "api/v2/Requests/Outgoing/Validate",
            Method::POST,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }

    pub async fn create_request(
        &self,
        req: &CreateOutgoingRequestRequest,
    ) -> Result<ConnectorRequest, crate::connector_errors::Error> {
        self.request(
            "api/v2/Requests/Outgoing",
            Method::POST,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }

    pub async fn get_request(
        &self,
        req_id: &str,
    ) -> Result<ConnectorRequest, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/Requests/Outgoing/{req_id}"),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn get_requests(
        &self,
        req: &GetRequestsRequest<'_>,
    ) -> Result<ConnectorRequest, crate::connector_errors::Error> {
        self.request(
            &format!(
                "api/v2/Requests/Outgoing?{}",
                serde_urlencoded::to_string(req).unwrap()
            ),
            Method::GET,
            None,
        )
        .await
    }
}
