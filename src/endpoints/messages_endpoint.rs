use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    types::{
        files::ConnectorFile,
        messages::{
            message::{ConnectorMessage, ConnectorMessageWithAttachments},
            requests::{GetMessagesRequest, SendMessageRequest},
        },
    },
    ConnectorClient,
};

/// All endpoints related to managing messages.
impl<'a> ConnectorClient<'a> {
    pub async fn get_messages(
        &self,
        attribute_query: &GetMessagesRequest,
    ) -> Result<Vec<ConnectorMessage<serde_json::Value>>, crate::connector_errors::Error> {
        self.request(
            &format!(
                "api/v2/Messages?{}",
                serde_urlencoded::to_string(attribute_query)
                    .expect("parsing of attribute to succeed")
            ),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn send_message<'i, T: Serialize + DeserializeOwned>(
        &self,
        send_message: &SendMessageRequest<'i, T>,
    ) -> Result<ConnectorMessage<T>, crate::connector_errors::Error> {
        self.request(
            "api/v2/Messages",
            Method::POST,
            Some(serde_json::to_string(send_message).unwrap()),
        )
        .await
    }

    pub async fn get_message<T: DeserializeOwned>(
        &self,
        id: &str,
    ) -> Result<ConnectorMessageWithAttachments<T>, crate::connector_errors::Error> {
        self.request(&format!("api/v2/Messages/{id}"), Method::GET, None)
            .await
    }

    pub async fn get_attachment(
        &self,
        message_id: &str,
        attachment_id: &str,
    ) -> Result<ConnectorFile, crate::connector_errors::Error> {
        self.request(
            &format!("api/v2/Messages/{message_id}/Attachments/{attachment_id}"),
            Method::GET,
            None,
        )
        .await
    }

    pub async fn get_attachment_data(
        &self,
        message_id: &str,
        attachment_id: &str,
    ) -> Result<Vec<u8>, crate::connector_errors::Error> {
        self.download(
            &format!("api/v2/Messages/{message_id}/Attachments/{attachment_id}/Download"),
            Method::GET,
        )
        .await
    }
}
