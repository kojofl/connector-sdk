use reqwest::Method;

use crate::{
    types::account::{ConnectorSyncResult, IdentityInfo, SyncInfo},
    ConnectorClient,
};

/// All endpoints related to the enmeshed account.
impl<'a> ConnectorClient<'a> {
    /// Retrieves the [`IdentityInfo`] of the connector the client is connected to.
    pub async fn get_identity_info(&self) -> Result<IdentityInfo, crate::connector_errors::Error> {
        self.request("api/v2/Account/IdentityInfo", Method::GET, None)
            .await
    }

    /// Retrieves the date of the last sync ran if there has been one.
    pub async fn get_sync_info(&self) -> Result<SyncInfo, crate::connector_errors::Error> {
        self.request("api/v2/Account/SyncInfo", Method::GET, None)
            .await
    }

    /// Syncs the connector with the backbone retrieving all messages and relationships that have
    /// been received since the last sync.
    pub async fn sync(&self) -> Result<ConnectorSyncResult, crate::connector_errors::Error> {
        self.request("api/v2/Account/Sync", Method::POST, None)
            .await
    }
}
