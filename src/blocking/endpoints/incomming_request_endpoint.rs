use reqwest::Method;

use super::Error;
use crate::{
    blocking::ConnectorClient,
    types::connector_request::{
        get_requests::GetRequestsRequest, requests::DecideRequest, ConnectorRequest,
        ConnectorRequestValidationResult,
    },
};

/// All endpoints related to managing incomming requests.
impl<'a> ConnectorClient<'a> {
    pub fn get_incomming_request(&self, id: &str) -> Result<Vec<ConnectorRequest>, Error> {
        self.request(&format!("api/v2/Requests/Incoming?{id}"), Method::GET, None)
    }

    pub fn get_incomming_requests(
        &self,
        req: &GetRequestsRequest<'_>,
    ) -> Result<Vec<ConnectorRequest>, Error> {
        self.request(
            &format!(
                "api/v2/Requests/Incoming?{}",
                serde_urlencoded::to_string(req).unwrap()
            ),
            Method::GET,
            None,
        )
    }

    pub fn can_accept(
        &self,
        request_id: &str,
        req: &DecideRequest,
    ) -> Result<ConnectorRequestValidationResult, Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/CanAccept"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
    }

    pub fn can_reject(
        &self,
        request_id: &str,
        req: &DecideRequest,
    ) -> Result<ConnectorRequestValidationResult, Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/CanReject"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
    }

    pub fn accept(&self, request_id: &str, req: &DecideRequest) -> Result<ConnectorRequest, Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/Accept"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
    }

    pub fn reject(&self, request_id: &str, req: &DecideRequest) -> Result<ConnectorRequest, Error> {
        self.request(
            &format!("api/v2/Requests/Incoming/{request_id}/Reject"),
            Method::PUT,
            Some(serde_json::to_string(req).unwrap()),
        )
    }
}
