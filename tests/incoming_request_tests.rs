use connector_sdk;

#[cfg(test)]
mod incomming_request_tests {
    use connector_sdk::types::connector_request::{
        get_requests::GetRequestsRequest, requests::DecideRequest, RequestContent,
    };

    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_get_incoming_requests() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let req = GetRequestsRequest {
            id: None,
            peer: Some("id1JtauzxRmZiBjvT6hXxse63Ve4bSqJmDt5"),
            ..GetRequestsRequest::default()
        };
        let res = client.get_incomming_requests(&req).await;
        println!("{:?}", res);
        assert!(res.is_ok())
    }

    #[actix_rt::test]
    async fn test_can_accept_incoming_request() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let req = GetRequestsRequest {
            peer: Some("id1JtauzxRmZiBjvT6hXxse63Ve4bSqJmDt5"),
            ..GetRequestsRequest::default()
        };
        let res = client.get_incomming_requests(&req).await;
        assert!(res.is_ok());
        let requests = res.unwrap();
        let request = &requests[0];
        let id = request.id.as_ref();
        // Accept all incomming requestsitems
        let r: DecideRequest = request
            .content
            .items
            .iter()
            .map(RequestContent::accept)
            .collect();
        let can_accept_res = client.can_accept(id, &r).await;
        assert!(can_accept_res.is_ok());
    }
}
