use connector_sdk;

#[cfg(test)]
mod monotoring_tests {
    use connector_sdk::types::relationship::requests::GetRelationshipsRequest;

    use super::connector_sdk::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_get_relationship() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let relationships_res = client
            .get_relationships(GetRelationshipsRequest::default())
            .await;
        println!("{:?}", relationships_res);
        assert!(relationships_res.is_ok());
    }
}
