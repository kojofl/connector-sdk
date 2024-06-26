use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use super::Error;
use crate::{
    blocking::ConnectorClient,
    types::{
        files::ConnectorFile,
        messages::{
            message::{ConnectorMessage, ConnectorMessageWithAttachments},
            requests::{GetMessagesRequest, SendMessageRequest},
        },
    },
};

/// All endpoints related to managing messages.
impl<'a> ConnectorClient<'a> {
    pub fn get_messages(
        &self,
        attribute_query: &GetMessagesRequest,
    ) -> Result<Vec<ConnectorMessage<serde_json::Value>>, Error> {
        self.request(
            &format!(
                "api/v2/Messages?{}",
                serde_urlencoded::to_string(attribute_query)
                    .expect("parsing of attribute to succeed")
            ),
            Method::GET,
            None,
        )
    }

    pub fn send_message<T: Serialize + DeserializeOwned>(
        &self,
        send_message: &SendMessageRequest<'_, T>,
    ) -> Result<ConnectorMessage<T>, Error> {
        self.request(
            "api/v2/Messages",
            Method::POST,
            Some(serde_json::to_string(send_message).unwrap()),
        )
    }

    pub fn get_message<T: DeserializeOwned>(
        &self,
        id: &str,
    ) -> Result<ConnectorMessageWithAttachments<T>, Error> {
        self.request(&format!("api/v2/Messages/{id}"), Method::GET, None)
    }

    pub fn get_attachment(
        &self,
        message_id: &str,
        attachment_id: &str,
    ) -> Result<ConnectorFile, Error> {
        self.request(
            &format!("api/v2/Messages/{message_id}/Attachments/{attachment_id}"),
            Method::GET,
            None,
        )
    }

    pub fn get_attachment_data(
        &self,
        message_id: &str,
        attachment_id: &str,
    ) -> Result<Vec<u8>, Error> {
        self.download(
            &format!("api/v2/Messages/{message_id}/Attachments/{attachment_id}/Download"),
            Method::GET,
        )
    }
}
