use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MinimalUser {
    pub username: String,
    pub password: String,
    pub is_owner: bool
}

#[derive(Serialize, Deserialize)]
pub struct WebID {
    pub user_reference_uri: String
}


#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

