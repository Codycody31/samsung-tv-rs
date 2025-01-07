use crate::commands::Command;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::net::TcpStream;
use std::time::Duration;
use tokio::time::sleep;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, Utf8Bytes, WebSocket};
use tungstenite::handshake::client::Response;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub FrameTVSupport: String,
    pub GamePadSupport: String,
    pub ImeSyncedSupport: String,
    pub OS: String,
    pub TokenAuthSupport: String,
    pub VoiceSupport: String,
    pub countryCode: String,
    pub description: String,
    pub developerIP: String,
    pub developerMode: String,
    pub duid: String,
    pub firmwareVersion: String,
    pub id: String,
    pub ip: String,
    pub model: String,
    pub modelName: String,
    pub name: String,
    pub networkType: String,
    pub resolution: String,
    pub smartHubAgreement: String,
    pub ssid: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub udn: String,
    pub wifiMac: String,
}

#[derive(Debug, Deserialize)]
pub struct SupportInfo {
    #[serde(flatten)]
    pub details: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct TVInfo {
    pub device: DeviceInfo,
    pub id: String,
    #[serde(rename = "isSupport", deserialize_with = "deserialize_support_info")]
    pub is_support: HashMap<String, String>,
    pub name: String,
    pub remote: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub uri: String,
    pub version: String,
}

fn deserialize_support_info<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let map: HashMap<String, String> =
        serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
    Ok(map)
}

pub struct SamsungTV {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    response: Response,
    key_interval: Duration,
    client: reqwest::Client,
    ip: String,
    port: u16,
    api_version: String,
}

impl SamsungTV {
    const URL_FORMAT: &'static str =
        "ws://{host}:{port}/api/v2/channels/samsung.remote.control?name={name}";

    pub async fn new(
        host: &str,
        port: u16,
        name: &str,
        api_version: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let url = Self::build_url(host, port, name)?;
        let (mut socket, response) = connect(url.to_string())?;

        Ok(Self {
            socket,
            response,
            key_interval: Duration::from_secs_f32(1.5),
            client: reqwest::Client::new(),
            ip: host.to_string(),
            port,
            api_version: api_version.to_string(),
        })
    }

    fn build_url(host: &str, port: u16, name: &str) -> Result<Url, url::ParseError> {
        let encoded_name = base64::encode(name);
        let url = Self::URL_FORMAT
            .replace("{host}", host)
            .replace("{port}", &port.to_string())
            .replace("{name}", &encoded_name);
        Url::parse(&url)
    }


    pub async fn get_info(&self) -> Result<TVInfo, Box<dyn std::error::Error>> {
        let url = format!("http://{}:{}/api/{}/", self.ip, self.port, self.api_version);

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<TVInfo>()
            .await?;

        Ok(response)
    }

    pub async fn send_command(
        &mut self,
        command: Command,
        repeat: usize,
    ) -> Result<Vec<Utf8Bytes>, Box<dyn std::error::Error>> {
        let mut responses = Vec::new();

        for _ in 0..repeat {
            let payload = json!({
                "method": "ms.remote.control",
                "params": {
                    "Cmd": "Click",
                    "DataOfCmd": command.as_str().to_string(),
                    "Option": "false",
                    "TypeOfRemote": "SendRemoteKey"
                }
            });

            self.socket
                .write(Message::Text(Utf8Bytes::from(payload.to_string())))?;

            // Wait for a response
            if let Ok(msg) = self.socket.read_message() {
                if let Message::Text(text) = msg {
                    responses.push(text);
                }
            }

            sleep(self.key_interval).await;
        }
        Ok(responses)
    }
}

impl Drop for SamsungTV {
    fn drop(&mut self) {
        if let Err(err) = self.socket.close(None) {
            eprintln!("Error closing connection: {:?}", err);
        }
    }
}

#[derive(serde::Serialize)]
struct CommandParams {
    Cmd: String,
    DataOfCmd: String,
    Option: String,
    TypeOfRemote: String,
}

#[derive(serde::Serialize)]
struct CommandPayload {
    method: String,
    params: CommandParams,
}
