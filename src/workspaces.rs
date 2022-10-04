use serde::{Deserialize, Serialize};
use anyhow::{Context, Result, bail};

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
    pub fn get(self) -> Result<Workspace> {
        let url = format!("{}/workspaces/get", self.1);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send get request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("workspace not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("{}", req.text().context("Really bad API failure")?);
        }

        let json: Root = req.json().context("Failed to deserialize JSON")?;
        Ok(json.workspace)
    }

    pub fn reset_sandbox(self) -> Result<()> {
        let url = format!("{}/workspaces/reset_sandbox", self.1);
        let header = format!("Bearer {}", self.0);
        let req = reqwest::blocking::Client::new()
            .post(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send request")?;

        if req.status() != reqwest::StatusCode::OK {
            bail!("Reset failed");
        }
        Ok(())
    }

    /// The /workspaces/list API isn't documented, but shows up in the SDK's
    pub fn list(self, workspace: Option<String>) -> Result<Vec<Workspace>> {
        let workspace_id = match workspace {
            Some(a) => a,
            None => "None".to_string(),
        };

        let url = format!("{}/workspaces/list?workspace_id={:?}", self.1, workspace_id);
        let header = format!("Bearer {}", self.0);

        let req = reqwest::blocking::Client::new()
            .get(url)
            .header("Authorization", header)
            .send()
            .context("Failed to send request")?;

        if req.status() == reqwest::StatusCode::NOT_FOUND {
            bail!("workspaces not found");
        } else if req.status() != reqwest::StatusCode::OK {
            bail!("request failed");
        }

        let json: ListRoot = req.json().context("Failed to deserialize JSON")?;
        Ok(json.workspaces)
    }
}
