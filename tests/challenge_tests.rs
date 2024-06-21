use connector_sdk;

#[cfg(test)]
mod challenge_tests {
    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_identity_challenge() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client.create_identity_challenge().await.is_ok());
    }

    #[actix_rt::test]
    async fn test_device_challenge() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client.create_device_challenge().await.is_ok());
    }
}
