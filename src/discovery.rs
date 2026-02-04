//! SSDP-based discovery for Samsung Smart TVs on the local network.
//!
//! This module provides functionality to automatically discover Samsung TVs
//! without needing to know their IP addresses in advance.

use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use crate::client::TvInfo;
use crate::error::{Result, SamsungTvError};

/// SSDP multicast address and port.
const SSDP_ADDR: Ipv4Addr = Ipv4Addr::new(239, 255, 255, 250);
const SSDP_PORT: u16 = 1900;

/// Default timeout for discovery.
const DEFAULT_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);

/// Information about a discovered Samsung TV.
#[derive(Debug, Clone)]
pub struct DiscoveredTv {
    /// IP address of the TV.
    pub ip: String,
    /// WebSocket port (typically 8001 or 8002).
    pub port: u16,
    /// Whether the TV supports secure connections (port 8002).
    pub supports_tls: bool,
    /// Location URL from SSDP response.
    pub location: Option<String>,
    /// Model name if available from SSDP.
    pub model: Option<String>,
    /// Friendly name if available from SSDP.
    pub name: Option<String>,
    /// Unique device ID.
    pub usn: Option<String>,
    /// Full TV info (fetched separately if requested).
    pub info: Option<TvInfo>,
}

impl DiscoveredTv {
    /// Creates a new DiscoveredTv with minimal info.
    fn new(ip: String) -> Self {
        Self {
            ip,
            port: 8001,
            supports_tls: false,
            location: None,
            model: None,
            name: None,
            usn: None,
            info: None,
        }
    }
}

/// Options for TV discovery.
#[derive(Debug, Clone)]
pub struct DiscoveryOptions {
    /// How long to wait for responses.
    pub timeout: Duration,
    /// Whether to fetch full TV info via HTTP after discovery.
    pub fetch_info: bool,
    /// Specific interface to bind to (None = all interfaces).
    pub bind_addr: Option<Ipv4Addr>,
}

impl Default for DiscoveryOptions {
    fn default() -> Self {
        Self {
            timeout: DEFAULT_DISCOVERY_TIMEOUT,
            fetch_info: false,
            bind_addr: None,
        }
    }
}

impl DiscoveryOptions {
    /// Creates new discovery options with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the discovery timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Whether to fetch full TV info after discovery.
    pub fn fetch_info(mut self, fetch: bool) -> Self {
        self.fetch_info = fetch;
        self
    }

    /// Sets a specific interface to bind to.
    pub fn bind_addr(mut self, addr: Ipv4Addr) -> Self {
        self.bind_addr = Some(addr);
        self
    }
}

/// Discovers Samsung TVs on the local network using SSDP.
///
/// # Example
///
/// ```no_run
/// use samsung_tv::discovery::{discover, DiscoveryOptions};
/// use std::time::Duration;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let options = DiscoveryOptions::new()
///     .timeout(Duration::from_secs(3))
///     .fetch_info(true);
///
/// let tvs = discover(options)?;
///
/// for tv in tvs {
///     println!("Found TV at {}:{}", tv.ip, tv.port);
///     if let Some(name) = &tv.name {
///         println!("  Name: {}", name);
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub fn discover(options: DiscoveryOptions) -> Result<Vec<DiscoveredTv>> {
    let bind_addr = options.bind_addr.unwrap_or(Ipv4Addr::UNSPECIFIED);
    let socket = UdpSocket::bind(SocketAddrV4::new(bind_addr, 0)).map_err(|e| {
        SamsungTvError::ConnectionFailed {
            host: "0.0.0.0".to_string(),
            port: 0,
            source: Box::new(e),
        }
    })?;

    socket.set_read_timeout(Some(options.timeout)).ok();
    socket.set_broadcast(true).ok();

    // Join multicast group
    socket.join_multicast_v4(&SSDP_ADDR, &bind_addr).ok();

    // Send M-SEARCH requests for Samsung TVs
    let search_targets = [
        "urn:samsung.com:device:RemoteControlReceiver:1",
        "urn:dial-multiscreen-org:service:dial:1",
        "ssdp:all",
    ];

    let dest = SocketAddr::V4(SocketAddrV4::new(SSDP_ADDR, SSDP_PORT));

    for st in &search_targets {
        let request = format!(
            "M-SEARCH * HTTP/1.1\r\n\
             HOST: 239.255.255.250:1900\r\n\
             MAN: \"ssdp:discover\"\r\n\
             MX: 3\r\n\
             ST: {}\r\n\
             \r\n",
            st
        );

        socket.send_to(request.as_bytes(), dest).ok();
    }

    // Collect responses
    let mut discovered: HashMap<String, DiscoveredTv> = HashMap::new();
    let mut buf = [0u8; 2048];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((len, addr)) => {
                let response = String::from_utf8_lossy(&buf[..len]);

                // Check if this is a Samsung TV response
                if is_samsung_tv(&response) {
                    let ip = addr.ip().to_string();

                    if let std::collections::hash_map::Entry::Vacant(e) =
                        discovered.entry(ip.clone())
                    {
                        let mut tv = DiscoveredTv::new(ip);
                        parse_ssdp_response(&response, &mut tv);

                        // Check if TV supports TLS by trying port 8002
                        tv.supports_tls = check_tls_support(&tv.ip);
                        if tv.supports_tls {
                            tv.port = 8002;
                        }

                        e.insert(tv);
                    }
                }
            }
            Err(e) => {
                // Timeout or other error - stop listening
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut
                {
                    break;
                }
            }
        }
    }

    let mut tvs: Vec<DiscoveredTv> = discovered.into_values().collect();

    // Optionally fetch full TV info
    if options.fetch_info {
        for tv in &mut tvs {
            tv.info = fetch_tv_info(&tv.ip, tv.port).ok();
        }
    }

    Ok(tvs)
}

/// Async version of discover that fetches TV info concurrently.
pub async fn discover_async(options: DiscoveryOptions) -> Result<Vec<DiscoveredTv>> {
    // Run SSDP discovery in blocking task
    let fetch_info = options.fetch_info;
    let mut tvs = tokio::task::spawn_blocking(move || {
        discover(DiscoveryOptions {
            fetch_info: false,
            ..options
        })
    })
    .await
    .map_err(|e| SamsungTvError::ConnectionFailed {
        host: "discovery".to_string(),
        port: 0,
        source: Box::new(e),
    })??;

    // Fetch TV info concurrently if requested
    if fetch_info {
        let client = reqwest::Client::new();
        let futures: Vec<_> = tvs
            .iter()
            .map(|tv| {
                let client = client.clone();
                let ip = tv.ip.clone();
                let port = tv.port;
                async move {
                    let url = format!("http://{}:{}/api/v2/", ip, port);
                    client
                        .get(&url)
                        .timeout(Duration::from_secs(3))
                        .send()
                        .await
                        .ok()
                        .and_then(|r| futures::executor::block_on(r.json::<TvInfo>()).ok())
                }
            })
            .collect();

        let results = futures::future::join_all(futures).await;

        for (tv, info) in tvs.iter_mut().zip(results) {
            tv.info = info;
        }
    }

    Ok(tvs)
}

/// Checks if an SSDP response is from a Samsung TV.
fn is_samsung_tv(response: &str) -> bool {
    let response_lower = response.to_lowercase();
    response_lower.contains("samsung")
        || response_lower.contains("tizen")
        || response_lower.contains("remotecontrolreceiver")
}

/// Parses SSDP response headers into DiscoveredTv fields.
fn parse_ssdp_response(response: &str, tv: &mut DiscoveredTv) {
    for line in response.lines() {
        let line = line.trim();
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_uppercase();
            let value = value.trim();

            match key.as_str() {
                "LOCATION" => {
                    tv.location = Some(value.to_string());
                    // Try to extract port from location URL
                    if let Ok(url) = url::Url::parse(value) {
                        if let Some(port) = url.port() {
                            // Samsung TV API is usually on 8001/8002, not the SSDP location port
                            if port == 8001 || port == 8002 {
                                tv.port = port;
                            }
                        }
                    }
                }
                "USN" => {
                    tv.usn = Some(value.to_string());
                }
                "SERVER" => {
                    // Try to extract model info from server string
                    if value.contains("Samsung") || value.contains("Tizen") {
                        tv.model = Some(value.to_string());
                    }
                }
                _ => {}
            }
        }
    }
}

/// Checks if a TV supports TLS by attempting a TCP connection to port 8002.
fn check_tls_support(ip: &str) -> bool {
    use std::net::TcpStream;

    let addr = format!("{}:8002", ip);
    TcpStream::connect_timeout(
        &addr
            .parse()
            .unwrap_or_else(|_| SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8002))),
        Duration::from_millis(500),
    )
    .is_ok()
}

/// Fetches TV info via HTTP API.
fn fetch_tv_info(ip: &str, port: u16) -> Result<TvInfo> {
    let url = format!("http://{}:{}/api/v2/", ip, port);

    let response = reqwest::blocking::Client::new()
        .get(&url)
        .timeout(Duration::from_secs(3))
        .send()
        .map_err(SamsungTvError::HttpError)?
        .json::<TvInfo>()
        .map_err(SamsungTvError::HttpError)?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_samsung_tv() {
        assert!(is_samsung_tv("SERVER: Samsung/1.0"));
        assert!(is_samsung_tv("Tizen TV"));
        assert!(is_samsung_tv("RemoteControlReceiver"));
        assert!(!is_samsung_tv("Roku"));
        assert!(!is_samsung_tv("Chromecast"));
    }

    #[test]
    fn test_parse_ssdp_response() {
        let response = "HTTP/1.1 200 OK\r\n\
            LOCATION: http://192.168.1.100:8001/\r\n\
            USN: uuid:abc123\r\n\
            SERVER: Samsung/Tizen UPnP/1.0\r\n";

        let mut tv = DiscoveredTv::new("192.168.1.100".to_string());
        parse_ssdp_response(response, &mut tv);

        assert_eq!(tv.location, Some("http://192.168.1.100:8001/".to_string()));
        assert_eq!(tv.usn, Some("uuid:abc123".to_string()));
        assert!(tv.model.is_some());
    }

    #[test]
    fn test_discovery_options_builder() {
        let options = DiscoveryOptions::new()
            .timeout(Duration::from_secs(10))
            .fetch_info(true)
            .bind_addr(Ipv4Addr::new(192, 168, 1, 50));

        assert_eq!(options.timeout, Duration::from_secs(10));
        assert!(options.fetch_info);
        assert_eq!(options.bind_addr, Some(Ipv4Addr::new(192, 168, 1, 50)));
    }
}
