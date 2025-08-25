use serde::{Deserialize, Serialize};
use super::FuzzyDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: Option<StaffName>,
    #[serde(rename = "languageV2")]
    pub language_v2: Option<String>,
    pub image: Option<StaffImage>,
    pub description: Option<String>,
    #[serde(rename = "primaryOccupations")]
    pub primary_occupations: Option<Vec<String>>,
    pub gender: Option<String>,
    #[serde(rename = "dateOfBirth")]
    pub date_of_birth: Option<FuzzyDate>,
    #[serde(rename = "dateOfDeath")]
    pub date_of_death: Option<FuzzyDate>,
    pub age: Option<i32>,
    #[serde(rename = "yearsActive")]
    pub years_active: Option<Vec<i32>>,
    #[serde(rename = "homeTown")]
    pub home_town: Option<String>,
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
pub struct StaffName {
    pub first: Option<String>,
    pub middle: Option<String>,
    pub last: Option<String>,
    pub full: Option<String>,
    pub native: Option<String>,
    pub alternative: Option<Vec<String>>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffImage {
    pub large: Option<String>,
    pub medium: Option<String>,
}
