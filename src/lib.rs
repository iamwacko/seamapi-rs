#![allow(unused)]

use anyhow::{Context, Result};

pub mod access_codes;
pub mod action_attempts;
pub mod connect_accounts;
pub mod connect_webviews;
pub mod devices;
pub mod events;
pub mod locks;
pub mod workspaces;

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
