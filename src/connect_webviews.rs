pub struct ConnectWebviews(pub String, pub String);

impl ConnectWebviews {
    pub fn list(self) {
        let url = format!("{}/connect_webviews/list", self.1);
        let headers = format!("Bearer {}", self.0);
        
        let req = reqwest::blocking::Client::new().get(url).header("Authorization", header).send().expect("Failed to send get request");

        if req.status() != reqwest::StatusCode::OK {
            panic!(res.text().expect("Text error"));
        }
    }

    pub fn get(self) {

    }

    pub fn create(self) {

    }
}
