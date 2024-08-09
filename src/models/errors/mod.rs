use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UnresolvedOwnerException {
    name: String,
    message: String
}

impl UnresolvedOwnerException {
    pub fn new() -> Self {
        Self {
            name: String::from("Unresolved Ownership Exception"),
            message: String::from("Server failed to resolve pod ownership at this time. Authorization unavailable.")
        }
    }
}