use serde::{Deserialize, Serialize};
use super::FuzzyDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: Option<CharacterName>,
    pub image: Option<CharacterImage>,
    pub description: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<FuzzyDate>,
    pub age: Option<String>,
    #[serde(rename = "bloodType")]
    pub blood_type: Option<String>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: Option<bool>,
    #[serde(rename = "isFavouriteBlocked")]
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    pub favourites: Option<i32>,
    #[serde(rename = "modNotes")]
    pub mod_notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterName {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: Option<String>,
    pub native: Option<String>,
    pub alternative: Option<Vec<String>>,
    pub alternative_spoiler: Option<Vec<String>>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}
