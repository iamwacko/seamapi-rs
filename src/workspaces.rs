use serde::{Deserialize, Serialize};

pub struct Workspaces(pub String, pub String);

#[derive(Serialize, Deserialize)]
struct Root {
    workspace: Workspace,
    ok: bool,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Workspace {
    pub workspace_id: String,
    pub name: String,
    pub connect_partner_name: String,
    pub is_sandbox: bool,
}

impl Workspaces {
    pub fn get(self) -> Workspace {
        let url = format!("{}/workspaces/get", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new().get(url).header("Authorization", header).send().expect("Failed to send get request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("workspace not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("{}", req.text().expect("Really bad API failure"));
        }
        
        let json: Root = req.json().expect("Failed to deserialize JSON");
        json.workspace
    }

    pub fn reset_sandbox(self) {
        let url = format!("{}/workspaces/reset_sandbox", self.1);
        let header = format!("Bearer {}", self.0);
        let req = reqwest::blocking::Client::new().post(url).header("Authorization", header).send().expect("Failed to send request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("Reset failed");
        }
    }

    pub fn list(self, workspace_id: Option<String>) {
        let mut query = String::new();
        let url = format!("{}/workspaces/list", self.1);
        let header = format!("Bearer {}", self.0);
        if workspace_id == None {
            query = "".to_string();
        } else {
            query = workspace_id.expect("Workspace ID in get failed");
        }

        let req = reqwest::blocking::Client::new().get(url).header("Authorization", header).query(&("workspace_id", query)).send().expect("Failed to send request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("workspaces not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("request failed");
        }
    } 
}
