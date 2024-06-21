use reqwest::Method;

use crate::{
    types::connector_request::{
        get_requests::GetRequestsRequest, requests::DecideRequest, ConnectorRequest,
        ConnectorRequestValidationResult,
    },
    ConnectorClient,
};

/// All endpoints related to managing incomming requests.
impl<'a> ConnectorClient<'a> {
    pub async fn get_incomming_request(
        &self,
        id: &str,
    ) -> Result<Vec<ConnectorRequest>, crate::connector_errors::Error> {
        self.request(&format!("api/v2/Requests/Incoming?{id}"), Method::GET, None)
            .await
    }

    pub async fn get_incomming_requests(
        &self,
        req: &GetRequestsRequest<'_>,
    ) -> Result<Vec<ConnectorRequest>, crate::connector_errors::Error> {
        self.request(
            &format!(
                "api/v2/Requests/Incoming?{}",
                serde_urlencoded::to_string(req).unwrap()
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn can_accept(
        &self,
        request_id: &str,
        req: &DecideRequest,
    ) -> Result<ConnectorRequestValidationResult, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/CanAccept"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }

    pub async fn can_reject(
        &self,
        request_id: &str,
        req: &DecideRequest,
    ) -> Result<ConnectorRequestValidationResult, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/CanReject"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }

    pub async fn accept(
        &self,
        request_id: &str,
        req: &DecideRequest,
    ) -> Result<ConnectorRequest, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/Accept"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }

    pub async fn reject(
        &self,
        request_id: &str,
        req: &DecideRequest,
    ) -> Result<ConnectorRequest, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/Reject"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
        .await
    }
}
