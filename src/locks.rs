use serde::{Deserialize, Serialize};

pub struct Locks(pub String, pub String);

struct Root {}

#[derive(Serialize, Deserialize)]
struct ListRoot {
    locks: Vec<crate::devices::Device>,
    ok: bool,
}

impl Locks {
    pub fn list(self) -> Vec<crate::devices::Device> {
        let url = format!("{}/locks/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"));
        }

        let json: ListRoot = req.json().expect("Failed to deserailize");
        json.locks
    }

    pub fn get(self) {
        let url = format!("{}/locks/get", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"));
        }
    }

    pub fn lock_door(self, device_id: String) -> crate::action_attempts::ActionAttempt {
        let url = format!("{}/locks/lock_door", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("device_id", device_id);

        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .expect("Failed to send post request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"));
        }

        let json: crate::action_attempts::Root = req.json().expect("Failed to deserialize JSON");
        json.action_attempt
    }

    pub fn unlock_door(self, device_id: String) -> crate::action_attempts::ActionAttempt {
        let url = format!("{}/locks/unlock_door", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("device_id", device_id);

        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .json(&map)
            .send()
            .expect("Failed to send post request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"));
        }

        let json: crate::action_attempts::Root = req.json().expect("Failed to deserialize JSON");
        json.action_attempt
    }
}
