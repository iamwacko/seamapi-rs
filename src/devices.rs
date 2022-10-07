use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub struct Devices(pub String, pub String);

#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    pub device_id: String,
    pub device_type: String,
    pub capabilities_supported: Vec<Value>,
    pub properties: Properties,
    pub location: Value,
    pub connected_account_id: String,
    pub workspace_id: String,
    pub created_at: String,
    pub errors: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Properties {
    pub locked: bool,
    pub online: bool,
    pub battery_level: f64,
    pub schlage_metadata: Option<SchlageMetadata>,
    pub august_metadata: Option<SchlageMetadata>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchlageMetadata {
    pub device_id: String,
    pub device_name: String,
}

impl Devices {
    pub fn get(self, device_id: String) -> Result<Device> {
        let url = format!("{}/devices/get?device_id={}", self.1, device_id);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failure")?)
        }

        let json: crate::Response = req.json().context("Failed to deserialize JSON")?;
        Ok(json.device.unwrap())
    }

    pub fn list(self) -> Result<Vec<Device>> {
        let url = format!("{}/devices/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!(
                "{}",
                req.text().context("Failed to turn response into text")?
            );
        }

        let json: crate::Response = req.json().context("Failed to deserialize JSON")?;
        Ok(json.devices.unwrap())
    }
}
