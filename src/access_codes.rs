use anyhow::{Result, Context, bail};
use serde::{Deserialize, Serialize};

pub struct AccessCodes(pub String, pub String);

#[derive(Serialize, Deserialize)]
struct Root {
    access_code: AccessCode,
    ok: bool,
}

#[derive(Serialize, Deserialize)]
struct ListRoot {
    access_codes: Vec<AccessCode>,
    ok: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessCode {
    pub code: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub status: String,
    pub created_at: String,
    pub access_code_id: String,
}

impl AccessCodes {
    pub fn list(self, device_id: String) -> Result<Vec<AccessCode>> {
        let url = format!("{}/access_codes/list?device_id={}", self.1, device_id);
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

        let json: ListRoot = req.json().context("Failed to deserialize JSON")?;
        Ok(json.access_codes)
    }

    pub fn get(self, access_code_id: String) -> Result<AccessCode> {
        let url = format!(
            "{}/access_codes/get?access_code_id={}",
            self.1, access_code_id
        );
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
        Ok(json.access_code)
    }

    pub fn create(
        self,
        device: String,
        name: Option<String>,
        code: Option<String>,
        starts_at: Option<String>,
        ends_at: Option<String>,
    ) -> Result<crate::action_attempts::ActionAttempt> {
        let url = format!("{}/access_codes/create", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("device_id", device);
        if name != None {
            map.insert("name", name.unwrap());
        }
        if code != None {
            map.insert("code", code.unwrap());
        }
        if starts_at != None {
            map.insert("starts_at", starts_at.unwrap());
        }
        if ends_at != None {
            map.insert("ends_at", ends_at.unwrap());
        }

        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .context("Failed to send post request")?;
        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API error")?);
        }

        let json: crate::action_attempts::Root = req.json().context("Failed to deserialize JSON")?;
        Ok(json.action_attempt)
    }

    pub fn delete(self, access_code_id: String) -> Result<crate::action_attempts::ActionAttempt> {
        let url = format!(
            "{}/access_codes/delete?access_code_id={}",
            self.1, access_code_id
        );
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .delete(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send delete request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("access code not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API error")?);
        }

        let json: crate::action_attempts::Root = req.json().context("Failed to deserialize JSON")?;
        Ok(json.action_attempt)
    }

    pub fn update(
        self,
        access_code_id: String,
        name: Option<String>,
        code: Option<String>,
        starts_at: Option<String>,
        ends_at: Option<String>,
    ) -> Result<crate::action_attempts::ActionAttempt> {
        let url = format!("{}/access_codes/update", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("access_code_id", access_code_id);
        if name != None {
            map.insert("name", name.unwrap());
        }
        if code != None {
            map.insert("code", code.unwrap());
        }
        if starts_at != None {
            map.insert("starts_at", starts_at.unwrap());
        }
        if ends_at != None {
            map.insert("ends_at", ends_at.unwrap());
        }

        let req = reqwest::blocking::Client::new()
            .put(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .context("failed to send put request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API error")?);
        }

        let json: crate::action_attempts::Root = req.json().context("Failed to deserialize JSON")?;
        Ok(json.action_attempt)
    }
}
