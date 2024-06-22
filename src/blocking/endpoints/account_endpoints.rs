use reqwest::Method;

use super::Error;
use crate::{
    blocking::ConnectorClient,
    types::account::{ConnectorSyncResult, IdentityInfo, SyncInfo},
};

/// All endpoints related to the enmeshed account.
impl<'a> ConnectorClient<'a> {
    /// Retrieves the [`IdentityInfo`] of the connector the client is connected to.
    pub fn get_identity_info(&self) -> Result<IdentityInfo, Error> {
        self.request("api/v2/Account/IdentityInfo", Method::GET, None)
    }

    /// Retrieves the date of the last sync ran if there has been one.
    pub fn get_sync_info(&self) -> Result<SyncInfo, Error> {
        self.request("api/v2/Account/SyncInfo", Method::GET, None)
    }

    /// Syncs the connector with the backbone retrieving all messages and relationships that have
    /// been received since the last sync.
    pub fn sync(&self) -> Result<ConnectorSyncResult, Error> {
        self.request("api/v2/Account/Sync", Method::POST, None)
    }
}
