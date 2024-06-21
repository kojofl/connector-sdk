use connector_sdk;

#[cfg(test)]
mod monotoring_tests {
    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_health() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client.check_health().await.is_ok());
    }

    #[actix_rt::test]
    async fn test_version() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client.check_version().await.is_ok());
    }

    #[actix_rt::test]
    async fn test_request_stats() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client.check_request_stats().await.is_ok());
    }

    #[actix_rt::test]
    async fn test_request_support() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client.get_support().await.is_ok());
    }
}
