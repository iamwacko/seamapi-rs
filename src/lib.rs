#![allow(unused)]

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub mod access_codes;
pub mod action_attempts;
pub mod connect_accounts;
pub mod connect_webviews;
pub mod devices;
pub mod events;
pub mod locks;
pub mod workspaces;

/// This struct isn't intended to be used directly
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub lock: Option<crate::devices::Device>,
    pub locks: Option<Vec<crate::devices::Device>>,
    pub workspace: Option<crate::workspaces::Workspace>,
    pub workspaces: Option<Vec<crate::workspaces::Workspace>>,
    pub action_attempt: Option<crate::action_attempts::ActionAttempt>,
    pub access_code: Option<crate::access_codes::AccessCode>,
    pub access_codes: Option<Vec<crate::access_codes::AccessCode>>,
    pub device: Option<crate::devices::Device>,
    pub devices: Option<Vec<crate::devices::Device>>,
    pub events: Option<Vec<crate::events::Event>>,
    pub connected_account: Option<crate::connect_accounts::ConnectedAccount>,
    pub connected_accounts: Option<Vec<crate::connect_accounts::ConnectedAccount>>,
    pub connect_webview: Option<crate::connect_webviews::ConnectWebview>,
    pub connect_webviews: Option<Vec<crate::connect_webviews::ConnectWebview>>,
    pub ok: bool,
}

#[derive(Clone, Copy)]
pub struct Seam {
    api_key: &'static str,
    api_url: &'static str,
}

impl Seam {
    pub fn new(api_key: Option<&str>, api_url: Option<&str>) -> Result<Self> {
        let mut key = String::new();
        let mut url = String::new();

        if api_key == None {
            key = std::env::var("SEAM_API_KEY")
                .context("SEAM_API_KEY not found in environment, and api_key not provided")?;
        } else {
            key = api_key.context("invalid API key")?.to_string();
        }

        if api_url == None {
            url = match std::env::var("SEAM_API_URL") {
                Ok(a) => a.to_string(),
                _ => "https://connect.getseam.com".to_string(),
            };
        } else {
            url = api_url.context("invalid URL")?.to_string();
        }

        let leak_key: &'static str = Box::leak(key.into_boxed_str());
        let leak_url: &'static str = Box::leak(url.into_boxed_str());
        Ok(Self {
            api_key: leak_key,
            api_url: leak_url,
        })
    }

    pub fn workspace(self) -> workspaces::Workspaces {
        workspaces::Workspaces(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn connected_accounts(self) -> connect_accounts::ConnectedAccounts {
        connect_accounts::ConnectedAccounts(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn connect_webviews(self) -> connect_webviews::ConnectWebviews {
        connect_webviews::ConnectWebviews(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn devices(self) -> devices::Devices {
        devices::Devices(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn locks(self) -> locks::Locks {
        locks::Locks(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn access_codes(self) -> access_codes::AccessCodes {
        access_codes::AccessCodes(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn action_attempts(self) -> action_attempts::ActionAttempts {
        action_attempts::ActionAttempts(self.api_key.to_string(), self.api_url.to_string())
    }

    pub fn events(self) -> events::Events {
        events::Events(self.api_key.to_string(), self.api_url.to_string())
    }
}
