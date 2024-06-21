use connector_sdk;

#[cfg(test)]
mod monotoring_tests {
    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_identity_info() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client.get_identity_info().await.is_ok());
    }

    #[actix_rt::test]
    async fn test_sync_info() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client.get_sync_info().await.is_ok());
    }

    #[actix_rt::test]
    async fn test_sync() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        assert!(client.sync().await.is_ok());
    }
}
