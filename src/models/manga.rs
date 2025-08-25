use serde::{Deserialize, Serialize};
use super::{MediaTitle, FuzzyDate, MediaFormat, MediaStatus, MediaSource, MediaCoverImage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub id: i32,
    pub title: Option<MediaTitle>,
    pub description: Option<String>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    #[serde(rename = "startDate")]
    pub start_date: Option<FuzzyDate>,
    #[serde(rename = "endDate")]
    pub end_date: Option<FuzzyDate>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub genres: Option<Vec<String>>,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i32>,
    #[serde(rename = "meanScore")]
    pub mean_score: Option<i32>,
    pub popularity: Option<i32>,
    pub favourites: Option<i32>,
    pub hashtag: Option<String>,
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<String>,
    #[serde(rename = "isAdult")]
    pub is_adult: Option<bool>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<MediaCoverImage>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    pub source: Option<MediaSource>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
}
