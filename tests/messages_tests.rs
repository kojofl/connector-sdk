use connector_sdk;

#[cfg(test)]
mod message_tests {
    use super::connector_sdk::ConnectorClient;

    use connector_sdk::types::messages::requests::{GetMessagesRequest, SendMessageRequest};
    use serde::{Deserialize, Serialize};

    static BASE_URL: &'static str = "http://localhost:8012";
    static API_KEY: &'static str = "xxx";
    #[actix_rt::test]
    async fn test_get_messages() {
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let get_messages_req = GetMessagesRequest::default();
        let res = client.get_messages(&get_messages_req).await;
        assert!(res.is_ok());
    }

    #[actix_rt::test]
    async fn test_send_message() {
        #[derive(Debug, Serialize, Deserialize)]
        struct Mail {
            header: String,
            content: String,
        }
        let client = ConnectorClient::new(BASE_URL, API_KEY);
        let send_message_req = SendMessageRequest {
            recipients: vec!["id1JtauzxRmZiBjvT6hXxse63Ve4bSqJmDt5"],
            content: Mail {
                header: "Test Mail".into(),
                content: "Hello there from the rust api.".into(),
            },
            attachments: None,
        };
        let res = client.send_message(&send_message_req).await;
        assert!(res.is_ok(), "{res:?}");
        let message = res.unwrap();
        assert_eq!(message.content.header, "Test Mail");
    }
}
