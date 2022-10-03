use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ConnectWebviews(pub String, pub String);

#[derive(Serialize, Deserialize)]
pub struct ConnectWebview {
    pub connect_webview_id: String,
    pub custom_redirect_url: Value,
    pub url: String,
    pub workspace_id: String,
    pub device_selection_mode: String,
    pub accepted_provider: Vec<String>,
    pub accepted_devices: Vec<Value>,
    pub any_provider_allowed: bool,
    pub any_device_allowed: Value,
    pub created_at: String,
    pub login_successful: bool,
    pub status: String,
}

#[derive(Deserialize)]
struct Root {
    connect_webview: ConnectWebview,
}

#[derive(Deserialize)]
struct ListRoot {
    connect_webviews: Vec<ConnectWebview>,
    ok: bool,
}

impl ConnectWebviews {
    pub fn list(self) -> Vec<ConnectWebview> {
        let url = format!("{}/connect_webviews/list", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Text error"));
        }
        let json: ListRoot = req.json().expect("failed to deserialize JSON");
        json.connect_webviews
    }

    pub fn get(self, connect_webview_id: String) -> ConnectWebview {
        let url = format!(
            "{}/connect_webview/get?connect_webview_id={}",
            self.1, connect_webview_id
        );
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("webview not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"))
        }

        let json: Root = req.json().expect("Failed to deserialize JSON");
        json.connect_webview
    }

    pub fn create(
        self,
        accepted_provider: Vec<String>,
        custom_redirect_url: Option<String>,
        device_selection_mode: Option<String>,
    ) -> ConnectWebview {
        let url = format!("{}/connect_webviews/create", self.1);
        let header = format!("Bearer {}", self.0);

        let mut map = std::collections::HashMap::new();
        map.insert("accepted_providers", accepted_provider);
        let mut map2 = std::collections::HashMap::new();
        if custom_redirect_url != None {
            map2.insert("custom_redirect_url", custom_redirect_url.unwrap());
        }
        if device_selection_mode != None {
            let d = device_selection_mode.unwrap();
            if d == "none".to_string() || d == "single".to_string() || d == "multiple".to_string() {
                map2.insert("device_selection_mode", d);
            }
        }

        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .json(&map)
            .json(&map2)
            .send()
            .expect("Failed to send post request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"));
        }

        let json: Root = req.json().expect("Failed to deserialize JSON");
        json.connect_webview
    }
}
