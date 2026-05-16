use serde::{Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldHeritageJson {
    pub name: String,
    pub description: String,
    pub region: String,
    pub state: String,
    pub coordinates: String,
}
