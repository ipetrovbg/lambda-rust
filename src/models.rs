use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Output {
    pub message: String,
    pub request_id: String,
}

#[derive(Deserialize)]
pub struct Event {
    pub message: String
}
