use reqwest::Method;

use crate::{
    blocking::ConnectorClient,
    types::monitoring::{
        ConnectorHealth, ConnectorRequestCount, ConnectorSupport, ConnectorVersionInfo,
    },
};
use super::Error;

/// All endpoints related to monotoring the connector status.
impl ConnectorClient<'_> {
    pub fn check_health(&self) -> Result<ConnectorHealth, Error> {
        self.request_plain::<ConnectorHealth>("health", Method::GET)
    }

    pub fn check_version(&self) -> Result<ConnectorVersionInfo, Error> {
        self.request_plain::<ConnectorVersionInfo>("Monitoring/Version", Method::GET)
    }

    pub fn check_request_stats(
        &self,
    ) -> Result<ConnectorRequestCount, Error> {
        self.request_plain::<ConnectorRequestCount>("Monitoring/Requests", Method::GET)
    }

    pub fn get_support(&self) -> Result<ConnectorSupport, Error> {
        self.request_plain::<ConnectorSupport>("Monitoring/Support", Method::GET)
    }
}
