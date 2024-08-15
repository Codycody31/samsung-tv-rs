use crate::commands::Command;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

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
    pub ip: String,
    pub port: u16,
    pub api_version: String,
    pub client: Client,
}

impl SamsungTV {
    pub fn new(ip: &str, port: u16, api_version: &str) -> Self {
        SamsungTV {
            ip: ip.to_string(),
            port,
            api_version: api_version.to_string(),
            client: Client::new(),
        }
    }

    pub async fn get_info(&self) -> Result<TVInfo, Box<dyn Error>> {
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

    pub async fn send_command(&self, command: Command) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "http://{}:{}/api/{}/channels/samsung.remote.control",
            self.ip, self.port, self.api_version
        );

        let payload = CommandPayload {
            method: "ms.remote.control".to_string(),
            params: CommandParams {
                Cmd: "Click".to_string(),
                DataOfCmd: command.as_str().to_string(),
                Option: "false".to_string(),
                TypeOfRemote: "SendRemoteKey".to_string(),
            },
        };

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        println!(
            "Sent command '{}' to Samsung TV at {}:{}",
            command.as_str(),
            self.ip,
            self.port
        );
        Ok(())
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
