use serde::{Deserialize, Serialize};

pub struct Workspaces(pub String, pub String);

#[derive(Serialize, Deserialize)]
struct Root {
    workspace: Workspace,
    ok: bool,
}

#[derive(Serialize, Deserialize)]
struct ListRoot {
    workspaces: Vec<Workspace>,
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

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send get request");

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
        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send request");

        if req.status() != reqwest::StatusCode::OK {
            panic!("Reset failed");
        }
    }

    /// The /workspaces/list API isn't documented, but shows up in the SDK's
    pub fn list(self, workspace: Option<String>) -> Vec<Workspace> {
        let mut workspace_id = "None".to_string();
        if workspace != None {
            workspace_id = workspace.unwrap();
        }
        let url = format!("{}/workspaces/list?workspace_id={:?}", self.1, workspace_id);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .expect("Failed to send request");

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            panic!("workspaces not found");
        } else if req.status() != reqwest::StatusCode::OK {
            panic!("request failed");
        }

        let json: ListRoot = req.json().expect("Failed to deserialize JSON");
        json.workspaces
    }
}
