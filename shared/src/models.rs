use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Film {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub title: String,
    pub director: String,
    pub year: u16,      // only positive numbers
    pub poster: String, // we will use the url of the poster here
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String,
}
