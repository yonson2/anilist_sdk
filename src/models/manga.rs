use serde::{Deserialize, Serialize};
use super::{MediaTitle, FuzzyDate, MediaFormat, MediaStatus, MediaSource, MediaCoverImage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub id: i32,
    pub title: Option<MediaTitle>,
    pub description: Option<String>,
    pub format: Option<MediaFormat>,
    pub status: Option<MediaStatus>,
    pub start_date: Option<FuzzyDate>,
    pub end_date: Option<FuzzyDate>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub genres: Option<Vec<String>>,
    pub average_score: Option<i32>,
    pub mean_score: Option<i32>,
    pub popularity: Option<i32>,
    pub favourites: Option<i32>,
    pub hashtag: Option<String>,
    pub country_of_origin: Option<String>,
    pub is_adult: Option<bool>,
    pub cover_image: Option<MediaCoverImage>,
    pub banner_image: Option<String>,
    pub source: Option<MediaSource>,
    pub updated_at: Option<i32>,
    pub site_url: Option<String>,
}
