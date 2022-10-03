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
    pub fn list(self) -> Vec<ConnectedAccount> {
        let url = format!("{}/connected_accounts/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API error"));
        }

        let json: ListRoot = req.json().expect("Failed to deserialize");
        json.connected_accounts
    }

    pub fn get(self, connected_account_id: String) -> ConnectedAccount {
        let url = format!(
            "{}/connected_accounts/get?connected_account_id={}",
            self.1, connected_account_id
        );
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API error"));
        }

        let json: Root = req.json().expect("Failed to deserialize JSON");
        json.connected_account
    }
}
