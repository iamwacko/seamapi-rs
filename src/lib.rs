#![allow(unused)]
pub mod devices;
pub mod workspaces;
pub mod connect_accounts;
pub mod locks;
pub mod access_codes;
pub mod action_attempts;
pub mod connect_webviews;

pub struct Seam {
    api_key: String,
    api_url: String,
}

impl Seam {
    pub fn new(api_key: Option<&str>, api_url: Option<&str>) -> Seam {
        let mut key = String::new();
        let mut url = String::new();

        if api_key == None {
            key = std::env::var("SEAM_API_KEY").expect("SEAM_API_KEY not found in environment, and api_key not provided");
        } else {
            key = api_key.expect("invalid API key").to_string();
        }
        if api_url == None {
            url = match std::env::var("SEAM_API_URL") {
                Ok(a) => a.to_string(),
                _ => "https://connect.getseam.com".to_string()
            };
        } else {
            url = api_url.expect("invalid URL").to_string();
        }
        Seam { api_key: key, api_url: url }
    }
    
    pub fn workspace(self) -> workspaces::Workspaces {
        workspaces::Workspaces(self.api_key.clone(), self.api_url.clone())
    }

    pub fn connected_accounts(self) -> connect_accounts::ConnectedAccounts{
        connect_accounts::ConnectedAccounts(self.api_key.clone(), self.api_url.clone())
    }

    pub fn connect_webviews(self) -> connect_webviews::ConnectWebviews{
        connect_webviews::ConnectWebviews(self.api_key.clone(), self.api_url.clone())
    }

    pub fn devices(self) -> devices::Devices{
        devices::Devices(self.api_key.clone(), self.api_url.clone())
    }

    pub fn locks(self) -> locks::Locks{
        locks::Locks(self.api_key.clone(), self.api_url.clone())
    }

    pub fn access_codes(self) -> access_codes::AccessCodes {
        access_codes::AccessCodes(self.api_key.clone(), self.api_url.clone())
    }

    pub fn action_attempts(self) -> action_attempts::ActionAttempts {
        action_attempts::ActionAttempts(self.api_key.clone(), self.api_url.clone())
    }
}


