use serde::{Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldHeritage {
    #[serde(rename = "Name EN")]
    pub name: String,

    #[serde(rename = "Description EN")]
    pub description: String,

    #[serde(rename = "Region")]
    pub region: String,

    #[serde(rename = "States Names")]
    pub state: String,

    #[serde(rename = "Coordinates")]
    pub coordinates: String,
}
