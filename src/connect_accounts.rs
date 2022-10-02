use serde::Deserialize;

pub struct ConnectedAccounts(pub String, pub String);

#[derive(Deserialize)]
pub struct ConnectedAccount {
    connected_accunt_id: String,
    created_at: String,
    user_identifier: String,
    account_type: String,
    errors: String,
}

impl ConnectedAccounts {
    pub fn list(self)  {
        todo!()
    }

    pub fn get(self) {
        todo!()
    }
}
