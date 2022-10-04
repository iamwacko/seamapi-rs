use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

pub struct Locks(pub String, pub String);

#[derive(Deserialize)]
struct Root {
    lock: crate::devices::Device,
    ok: bool,
}

#[derive(Serialize, Deserialize)]
struct ListRoot {
    locks: Vec<crate::devices::Device>,
    ok: bool,
}

impl Locks {
    pub fn list(self) -> Result<Vec<crate::devices::Device>> {
        let url = format!("{}/locks/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failure")?);
        }

        let json: ListRoot = req.json().context("Failed to deserailize")?;
        Ok(json.locks)
    }

    pub fn get(self, device_id: String) -> Result<crate::devices::Device> {
        let url = format!("{}/locks/get?device_id={}", self.1, device_id);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failure")?);
        }

        let json: Root = req.json().context("Failed to deserialize JSON")?;
        Ok(json.lock)
    }

    pub fn lock_door(self, device_id: String) -> Result<crate::action_attempts::ActionAttempt> {
        let url = format!("{}/locks/lock_door", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("device_id", device_id);

        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .context("Failed to send post request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failure")?);
        }

        let json: crate::action_attempts::Root =
            req.json().context("Failed to deserialize JSON")?;
        Ok(json.action_attempt)
    }

    pub fn unlock_door(self, device_id: String) -> Result<crate::action_attempts::ActionAttempt> {
        let url = format!("{}/locks/unlock_door", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("device_id", device_id);

        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .context("Failed to send post request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failure")?);
        }

        let json: crate::action_attempts::Root =
            req.json().context("Failed to deserialize JSON")?;
        Ok(json.action_attempt)
    }
}
