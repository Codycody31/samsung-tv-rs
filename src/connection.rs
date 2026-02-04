//! WebSocket connection management for Samsung TVs.

use std::sync::Arc;

use base64::Engine;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{client_async_tls_with_config, Connector, MaybeTlsStream, WebSocketStream};
use tracing::{debug, warn};
use url::Url;

use crate::auth::TokenManager;
use crate::config::SamsungTvConfig;
use crate::error::{Result, SamsungTvError};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSink = SplitSink<WsStream, Message>;
type WsSource = SplitStream<WsStream>;

/// Manages the WebSocket connection to a Samsung TV.
pub struct Connection {
    sink: WsSink,
    stream: WsSource,
    config: Arc<SamsungTvConfig>,
    connected: bool,
}

impl Connection {
    /// Establishes a new connection to a Samsung TV.
    pub async fn connect(config: &SamsungTvConfig) -> Result<(Self, Option<String>)> {
        let url = Self::build_url(config)?;
        debug!("Connecting to Samsung TV at {}", url);

        // Build TLS connector that accepts self-signed certificates (Samsung TVs use them)
        let connector = if config.use_tls {
            let tls = native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
                .map_err(SamsungTvError::TlsError)?;
            Some(Connector::NativeTls(tls))
        } else {
            None
        };

        // Create the request
        let request = url.as_str().into_client_request()
            .map_err(SamsungTvError::WebSocketError)?;

        // Connect to the TV
        let tcp_stream = TcpStream::connect((config.host.as_str(), config.port))
            .await
            .map_err(|e| SamsungTvError::ConnectionFailed {
                host: config.host.clone(),
                port: config.port,
                source: Box::new(e),
            })?;

        let connect_future = client_async_tls_with_config(
            request,
            tcp_stream,
            None,
            connector,
        );

        let (socket, _response) = timeout(config.timeout, connect_future)
            .await
            .map_err(|_| SamsungTvError::Timeout)?
            .map_err(SamsungTvError::WebSocketError)?;

        let (sink, stream) = socket.split();

        let mut conn = Self {
            sink,
            stream,
            config: Arc::new(config.clone()),
            connected: true,
        };

        // Wait for the connection response and extract token if present
        let token = conn.wait_for_connection_response().await?;

        Ok((conn, token))
    }

    /// Builds the WebSocket URL for connecting to the TV.
    fn build_url(config: &SamsungTvConfig) -> Result<Url> {
        let encoded_name = base64::engine::general_purpose::STANDARD.encode(&config.name);

        let mut url = format!(
            "{}://{}:{}/api/v2/channels/samsung.remote.control?name={}",
            config.scheme(),
            config.host,
            config.port,
            encoded_name
        );

        // Add token if available (required for secure connections)
        if let Some(token) = &config.token {
            url.push_str("&token=");
            url.push_str(token);
        }

        Url::parse(&url).map_err(SamsungTvError::UrlParseError)
    }

    /// Waits for the initial connection response from the TV.
    ///
    /// Returns the authentication token if one is provided.
    async fn wait_for_connection_response(&mut self) -> Result<Option<String>> {
        let timeout_duration = self.config.timeout;

        match timeout(timeout_duration, self.stream.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => {
                let text_str = text.to_string();
                debug!("Connection response: {}", text_str);

                if TokenManager::is_auth_error(&text_str) {
                    return Err(SamsungTvError::AuthenticationFailed);
                }

                if TokenManager::is_connected(&text_str) {
                    return Ok(TokenManager::extract_from_response(&text_str));
                }

                Ok(None)
            }
            Ok(Some(Ok(_))) => Ok(None),
            Ok(Some(Err(e))) => Err(SamsungTvError::WebSocketError(e)),
            Ok(None) => Err(SamsungTvError::InvalidResponse {
                message: "Connection closed immediately".to_string(),
            }),
            Err(_) => Err(SamsungTvError::Timeout),
        }
    }

    /// Sends a text message through the WebSocket.
    pub async fn send(&mut self, message: &str) -> Result<()> {
        if !self.connected {
            return Err(SamsungTvError::NotConnected);
        }

        self.sink
            .send(Message::Text(message.to_string()))
            .await
            .map_err(SamsungTvError::WebSocketError)?;

        Ok(())
    }

    /// Receives the next message from the WebSocket.
    ///
    /// Returns `None` if the connection is closed.
    pub async fn receive(&mut self) -> Result<Option<String>> {
        if !self.connected {
            return Err(SamsungTvError::NotConnected);
        }

        match self.stream.next().await {
            Some(Ok(Message::Text(text))) => Ok(Some(text.to_string())),
            Some(Ok(Message::Close(_))) => {
                self.connected = false;
                Ok(None)
            }
            Some(Ok(_)) => Ok(None), // Ignore ping/pong/binary
            Some(Err(e)) => {
                self.connected = false;
                Err(SamsungTvError::WebSocketError(e))
            }
            None => {
                self.connected = false;
                Ok(None)
            }
        }
    }

    /// Receives a message with a timeout.
    pub async fn receive_timeout(&mut self, duration: Duration) -> Result<Option<String>> {
        match timeout(duration, self.receive()).await {
            Ok(result) => result,
            Err(_) => Ok(None), // Timeout is not an error, just no message
        }
    }

    /// Closes the WebSocket connection gracefully.
    pub async fn close(&mut self) -> Result<()> {
        if self.connected {
            self.connected = false;
            self.sink
                .send(Message::Close(None))
                .await
                .map_err(SamsungTvError::WebSocketError)?;
        }
        Ok(())
    }

    /// Returns whether the connection is still open.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Attempts to reconnect using the stored configuration.
    #[allow(dead_code)]
    pub async fn reconnect(&mut self) -> Result<Option<String>> {
        if self.connected {
            warn!("Reconnect called while still connected, closing first");
            let _ = self.close().await;
        }

        let url = Self::build_url(&self.config)?;
        debug!("Reconnecting to Samsung TV at {}", url);

        // Build TLS connector that accepts self-signed certificates
        let connector = if self.config.use_tls {
            let tls = native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
                .map_err(SamsungTvError::TlsError)?;
            Some(Connector::NativeTls(tls))
        } else {
            None
        };

        let request = url.as_str().into_client_request()
            .map_err(SamsungTvError::WebSocketError)?;

        let tcp_stream = TcpStream::connect((self.config.host.as_str(), self.config.port))
            .await
            .map_err(|e| SamsungTvError::ConnectionFailed {
                host: self.config.host.clone(),
                port: self.config.port,
                source: Box::new(e),
            })?;

        let connect_future = client_async_tls_with_config(
            request,
            tcp_stream,
            None,
            connector,
        );

        let (socket, _response) = timeout(self.config.timeout, connect_future)
            .await
            .map_err(|_| SamsungTvError::Timeout)?
            .map_err(SamsungTvError::WebSocketError)?;

        let (sink, stream) = socket.split();
        self.sink = sink;
        self.stream = stream;
        self.connected = true;

        self.wait_for_connection_response().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url_insecure() {
        let config = SamsungTvConfig::new("192.168.1.100")
            .name("TestClient")
            .build();

        let url = Connection::build_url(&config).unwrap();
        assert!(url.as_str().starts_with("ws://"));
        assert!(url.as_str().contains("192.168.1.100:8001"));
        assert!(url.as_str().contains("name="));
        assert!(!url.as_str().contains("token="));
    }

    #[test]
    fn test_build_url_secure_with_token() {
        let config = SamsungTvConfig::new("192.168.1.100")
            .secure()
            .name("TestClient")
            .token("abc123")
            .build();

        let url = Connection::build_url(&config).unwrap();
        assert!(url.as_str().starts_with("wss://"));
        assert!(url.as_str().contains("192.168.1.100:8002"));
        assert!(url.as_str().contains("token=abc123"));
    }
}
