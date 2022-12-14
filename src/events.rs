use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

pub struct Events(pub String, pub String);

#[derive(Serialize, Deserialize)]
pub struct Event {
    pub event_id: String,
    pub event_type: String,
    pub created_at: String,
    pub device_id: String,
    pub workspace_id: String,
    pub battery_level: Option<f64>,
}

impl Events {
    pub fn list(self, since: String, device_id: Option<String>) -> Result<Vec<Event>> {
        let url = format!("{}/events/list", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("since", since);
        if device_id != None {
            map.insert("device_id", device_id.unwrap());
        }

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .context("Failed to send get request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API error")?);
        }

        let json: crate::Response = req.json().context("Failed to deserialize JSON")?;
        Ok(json.events.unwrap())
    }
}
