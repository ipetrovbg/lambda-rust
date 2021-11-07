use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Output {
    pub message: String,
    pub request_id: String,
}

#[derive(Deserialize)]
pub struct Event {
    pub first_name: String,
    pub second_name: String,
}