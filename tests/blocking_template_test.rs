use connector_sdk;

#[cfg(test)]
mod template_test {
    use super::connector_sdk::blocking::ConnectorClient;

    static BASE_URL: &'static str = "http://localhost:5012";
    static API_KEY: &'static str = "xxx";

    #[test]
    fn test_get_own_relationship_template() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        assert!(client.get_own_relationship_templates(None).is_ok());
    }

    #[test]
    fn test_get_qr_for_relationship_template() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);

        let v = client.get_qr_code_for_relationship_template("RLTDX3Wpg7viffQIK1kd");
        println!("{v:?}");
        assert!(v.is_ok())
    }
}
