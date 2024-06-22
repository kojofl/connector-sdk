use crate::connector_errors::ConnectorError;

#[derive(Debug)]
pub enum Error {
    RequestError(Box<ureq::Error>),
    ConnectorError(ConnectorError),
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        Error::RequestError(Box::new(e))
    }
}

impl From<ConnectorError> for Error {
    fn from(e: ConnectorError) -> Self {
        Error::ConnectorError(e)
    }
}
