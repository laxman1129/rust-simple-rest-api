use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}
