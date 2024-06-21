use chrono::Utc;
use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ConnectorError(ConnectorError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::RequestError(e)
    }
}

impl From<ConnectorError> for Error {
    fn from(e: ConnectorError) -> Self {
        Error::ConnectorError(e)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConnectorError {
    pub id: String,
    pub code: String,
    pub message: String,
    pub docs: String,
    pub time: chrono::DateTime<Utc>,
}
