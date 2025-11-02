/// Host implementation.
pub struct Host {
}

impl Default for Host {
    fn default() -> Self {
        todo!();
    }
}

impl Host {

    /// Connect to an MCP server.
    pub async fn connect(&self) {
        todo!();
    }

    /// Disconnect a specific MCP server.
    pub async fn disconnect(&self) {
        todo!();
    }

    /// Disconnect from all servers.
    pub async fn disconnect_all(&self) {
        todo!();
    }

    /// List the servers the host is connected to.
    pub async fn list_connected(&self) {
        todo!();
    }

    /// Get the capabilities of a given server.
    ///
    /// The server need not be [connected](Self::connect). If the
    /// server is not already connected, the server's capabilities are
    /// fetched and the connection is immediately dropped.
    pub async fn get_server_capabilities(&self) {
        todo!();
    }

    /// Get the server's MCP protocol version.
    ///
    /// The server need not be [connected](Self::connect). If the
    /// server is not already connected, the server's protocol version
    /// is fetched and the connection is immediately dropped.
    pub async fn get_server_protocol_version(&self) {
        todo!();
    }

    /// Get the server information object.
    ///
    /// The server need not be [connected](Self::connect). If the
    /// server is not already connected, the server's info is fetched
    /// and the connection is immediately dropped.
    pub async fn get_server_info(&self) {
        todo!();
    }

    /// List prompts exposed by connected servers.
    ///
    /// Calls the `prompts/list` method on all servers that support
    /// the prompts capability and collects their responses into a
    /// single vector.
    pub async fn list_prompts(&self) {
        todo!();
    }

    /// Get a specific prompt.
    ///
    /// Calls the `prompts/get` method on the server that owns the
    /// specified prompt.
    pub async fn get_prompt(&self) {
        todo!();
    }

    /// List resources exposed by connected servers.
    ///
    /// Calls the `resources/list` method on all servers that support
    /// the resources capability and collects their responses into a
    /// single vector.
    pub async fn list_resources(&self) {
        todo!();
    }

    /// Retrieve resource contents.
    ///
    /// Sends a `resource/read` request to the server that owns the
    /// specified resource.
    pub async fn read_resource(&self) {
        todo!();
    }

    /// List resource templates exposed by connected servers.
    ///
    /// Calls the `resources/templates/list` method on all servers
    /// that support the resource templates capability and collects
    /// their responses into a single vector.
    pub async fn list_resource_templates(&self) {
        todo!();
    }

    /// List tools exposed by connected servers.
    ///
    /// Calls the `tools/list` method on all servers that support the
    /// tools capability and collects their responses into a single
    /// vector.
    pub async fn list_tools(&self) {
        todo!();
    }

    /// Invoke a specific tool.
    ///
    /// Sends a `tools/call` request to the server that owns the
    /// specified tool.
    pub async fn call_tool(&self) {
        todo!();
    }

    /// List the latest notifications.
    ///
    /// Lists the latest notifications from all managed servers.
    pub async fn list_notifications(&self) {
        todo!();
    }

    /// List notifications from a specific server.
    pub async fn list_server_notifications(&self) {
        todo!();
    }

    /// Get a stream to listen to notifications from all servers.
    pub async fn get_notification_stream(&self) {
        todo!();
    }

    /// Get a stream to listen to notifications from a specific
    /// server.
    pub async fn get_server_notification_stream(&self) {
        todo!();
    }
}
