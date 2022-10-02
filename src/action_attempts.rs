use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ActionAttempts(pub String, pub String);

#[derive(Deserialize, Serialize)]
struct Root {
    pub action_attempt: ActionAttempt,
    pub ok: bool,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ActionAttempt {
    pub status: String,
    pub action_type: String,
    pub action_attempt_id: String,
    pub result: Value,
    pub error: Value,
}

impl ActionAttempts {
    pub fn get(self) -> ActionAttempt {
        let url = format!("{}/action_attempts/get", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new().get(url).header("authorization", header).send().expect("Failed to send get request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("Action Attempt not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failue"));
        }

        let json: Root = req.json().expect("Failed to deserialize");
        json.action_attempt
    }

    pub fn poll_until_ready(self) {
        todo!()
    }
}
