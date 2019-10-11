use serde::{Deserialize, Serialize};

pub mod email;
pub mod slack;

#[derive(Serialize, Deserialize)]
struct ResponseOk {
    result: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseError {
    error: String,
}
