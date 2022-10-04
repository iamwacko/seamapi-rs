use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ConnectedAccounts(pub String, pub String);

#[derive(Deserialize)]
struct Root {
    connected_account: ConnectedAccount,
    ok: bool,
}

#[derive(Deserialize)]
struct ListRoot {
    connected_accounts: Vec<ConnectedAccount>,
    ok: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ConnectedAccount {
    connected_accunt_id: String,
    created_at: String,
    user_identifier: UserIdentifier,
    account_type: String,
    errors: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct UserIdentifier {
    pub email: String,
}

impl ConnectedAccounts {
    pub fn list(self) -> Result<Vec<ConnectedAccount>> {
        let url = format!("{}/connected_accounts/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API error")?);
        }

        let json: ListRoot = req.json().context("Failed to deserialize JSON")?;
        Ok(json.connected_accounts)
    }

    pub fn get(self, connected_account_id: String) -> Result<ConnectedAccount> {
        let url = format!(
            "{}/connected_accounts/get?connected_account_id={}",
            self.1, connected_account_id
        );
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API error")?);
        }

        let json: Root = req.json().context("Failed to deserialize JSON")?;
        Ok(json.connected_account)
    }
}
