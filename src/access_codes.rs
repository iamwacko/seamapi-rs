use serde::{Serialize, Deserialize};

pub struct AccessCodes(pub String, pub String);

#[derive(Serialize, Deserialize)]
struct Root {
    access_code: AccessCode,
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
    pub fn list(self) {
        todo!()
    }

    pub fn get(self) {
        todo!()
    }

    pub fn create(self) {
        todo!()
    }

    pub fn delete(self) {
        todo!()
    }
}
