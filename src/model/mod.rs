use serde:: {Deserialize, Serialize};

// data model
#[derive(Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub genre: String
}