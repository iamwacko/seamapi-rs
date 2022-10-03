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

#[derive(Serialize, Deserialize)]
pub struct AccessCode {
    pub code: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub status: String,
    pub created_at: String,
    pub access_code_id: String,
}

impl AccessCodes {
    pub fn list(self, device_id: String) -> Vec<AccessCode> {
        let url = format!("{}/access_codes/list?device_id={}", self.1, device_id);
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

        let json: ListRoot = req.json().expect("Failed to deserialize JSON");
        json.access_codes
    }

    pub fn get(self, access_code_id: String) -> AccessCode {
        let url = format!(
            "{}/access_codes/get?access_code_id={}",
            self.1, access_code_id
        );
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
        let json: Root = req.json().expect("Failed to deserialize JSON");
        json.access_code
    }

    pub fn create(
        self,
        device: String,
        name: Option<String>,
        code: Option<String>,
        starts_at: Option<String>,
        ends_at: Option<String>,
    ) -> crate::action_attempts::ActionAttempt {
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
            .expect("Failed to send post request");
        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("device not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API error"));
        }

        let json: crate::action_attempts::Root = req.json().expect("Failed to deserialize JSON");
        json.action_attempt
    }

    pub fn delete(self, access_code_id: String) {
        let url = format!(
            "{}/access_codes/delete?access_code_id={}",
            self.1, access_code_id
        );
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .delete(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send delete request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("access code not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API error"));
        }

        let json: crate::action_attempts::Root = req.json().expect("Failed to deserialize JSON");
    }

    pub fn update(
        self,
        access_code_id: String,
        name: Option<String>,
        code: Option<String>,
        starts_at: Option<String>,
        ends_at: Option<String>,
    ) -> crate::action_attempts::ActionAttempt {
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
            .expect("failed to send put request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API error"));
        }

        let json: crate::action_attempts::Root = req.json().expect("Failed to deserialize JSON");
        json.action_attempt
    }
}
