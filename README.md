The connector-sdk crate exposes a http client that serves as a wrapper around the HTTP-api of the
enmeshed connector. In addition it exposes sane type definitions for request and return values 
implement some convenience functions for handeling common operations.

# Example
```rust
use connector_sdk::ConnectorClient;

#[tokio::main]
async fn main() {
    let client = ConnectorClient::new(BASE_URL, API_KEY);
    let req = GetRequestsRequest::default();
    let incomming_requests = client
        .get_incomming_requests(&req)
        .await
        .expect("receive incomming requests");
    let request = &incomming_requests[0];
    let id = request.id;
    // Accept all incomming requestsitems
    let r: DecideRequest = request
        .content
        .items
        .iter()
        .map(RequestContent::accept)
        .collect();
    let can_accept_res = client.can_accept(id.as_ref(), &r).await;
    assert!(can_accept_res.is_ok());
}
```
