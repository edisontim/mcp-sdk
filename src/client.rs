use crate::{
    protocol::{Protocol, ProtocolBuilder, RequestOptions},
    transport::Transport,
    types::{
        ClientCapabilities, Implementation, InitializeRequest, InitializeResponse,
        LATEST_PROTOCOL_VERSION,
    },
};

use anyhow::{Context, Result};
use tracing::debug;

#[derive(Clone)]
pub struct Client<T: Transport> {
    protocol: Protocol<T>,
}

impl<T: Transport> Client<T> {
    pub fn builder(transport: T) -> ClientBuilder<T> {
        ClientBuilder::new(transport)
    }

    pub async fn initialize(&self, client_info: Implementation) -> Result<InitializeResponse> {
        let request = InitializeRequest {
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities::default(),
            client_info,
        };

        let request_value =
            serde_json::to_value(request).context("Failed to serialize initialize request")?;

        let response = self
            .request("initialize", Some(request_value), RequestOptions::default())
            .await
            .context("Failed to send initialize request")?;

        let response: InitializeResponse =
            serde_json::from_value(response).context("Failed to parse initialize response")?;

        if response.protocol_version != LATEST_PROTOCOL_VERSION {
            return Err(anyhow::anyhow!(
                "Unsupported protocol version: expected {}, got {}",
                LATEST_PROTOCOL_VERSION,
                response.protocol_version
            ));
        }

        debug!(
            "Initialized with protocol version: {}",
            response.protocol_version
        );

        self.protocol
            .notify("notifications/initialized", None)
            .context("Failed to send initialized notification")?;

        Ok(response)
    }

    pub async fn request(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
        options: RequestOptions,
    ) -> Result<serde_json::Value> {
        let response = self
            .protocol
            .request(method, params, options)
            .await
            .with_context(|| format!("Request failed for method: {}", method))?;

        response.result.ok_or_else(|| {
            let error_msg = response
                .error
                .as_ref()
                .map(|e| format!("{}: {}", e.code, e.message))
                .unwrap_or_else(|| "Unknown error".to_string());

            anyhow::anyhow!("Request '{}' failed: {}", method, error_msg)
        })
    }

    pub async fn start(&self) -> Result<()> {
        self.protocol
            .listen()
            .await
            .context("Client protocol listener failed")
    }
}

pub struct ClientBuilder<T: Transport> {
    protocol: ProtocolBuilder<T>,
}

impl<T: Transport> ClientBuilder<T> {
    pub fn new(transport: T) -> Self {
        Self {
            protocol: ProtocolBuilder::new(transport),
        }
    }

    pub fn build(self) -> Client<T> {
        Client {
            protocol: self.protocol.build(),
        }
    }
}
