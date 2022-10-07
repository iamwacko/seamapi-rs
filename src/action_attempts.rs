use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ActionAttempts(pub String, pub String);

#[derive(Deserialize, Serialize, Debug)]
pub struct ActionAttempt {
    pub status: String,
    pub action_type: String,
    pub action_attempt_id: String,
    pub result: Value,
    pub error: Value,
}

impl ActionAttempts {
    pub fn get(self, action_attempt_id: String) -> Result<ActionAttempt> {
        let url = format!(
            "{}/action_attempts/get?action_attempt_id={}",
            self.1, action_attempt_id
        );
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("Action Attempt not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failue")?);
        }

        let json: crate::Response = req.json().context("Failed to deserialize")?;
        Ok(json.action_attempt.unwrap())
    }
}
