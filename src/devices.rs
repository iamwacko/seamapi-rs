use serde::{Deserialize, Serialize};
use serde_json::Value;
pub struct Devices(pub String, pub String);

#[derive(Serialize, Deserialize)]
struct Root {
    device: Device,
    ok: bool,
}

#[derive(Serialize, Deserialize)]
struct DeviceList {
    devices: Vec<Device>,
    ok: bool,
}

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
    pub schlage_metadata: SchlageMetadata,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SchlageMetadata {
    pub device_id: String,
    pub device_name: String,
}

impl Devices {
    pub fn get(self, device_id: String) -> Device {
        let url = format!("{}/devices/get?device_id={}", self.1, device_id);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"))
        }

        let json: Root = req.json().expect("Failed to deserialize JSON");
        json.device
    }

    pub fn list(self) -> Vec<Device> {
        let url = format!("{}/devices/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() != reqwest::StatusCode::OK {
            panic!();
        }

        let json: DeviceList = req.json().expect("Failed to deserialize JSON");
        json.devices
    }
}
