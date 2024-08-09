use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ResourceRequestQueryInfo {
    pub resource_iri: String,
}
