//! # Anime Data Models
//!
//! This module contains data structures representing anime information
//! as returned by the AniList API.

use serde::{Deserialize, Serialize};

/// Represents a complete anime entry from AniList.
///
/// This struct contains comprehensive information about an anime series or movie,
/// including metadata, statistics, dates, and relationships. All fields are optional
/// as different API endpoints may return varying levels of detail.
///
/// # Field Descriptions
///
/// ## Identification
/// - `id`: Unique AniList identifier for this anime
/// - `title`: Multi-language title information (romaji, english, native)
/// - `hashtag`: Official hashtag used for social media
///
/// ## Content Information  
/// - `description`: Synopsis or description (may contain HTML)
/// - `format`: Type of anime (TV, Movie, OVA, etc.)
/// - `status`: Current status (Airing, Finished, etc.)
/// - `genres`: List of genre tags
/// - `episodes`: Total number of episodes (null for ongoing series)
/// - `duration`: Average episode duration in minutes
/// - `is_adult`: Whether the content is marked as adult/mature
/// - `country_of_origin`: Country where the anime was produced
///
/// ## Scheduling
/// - `start_date`: When the anime began airing
/// - `end_date`: When the anime finished airing (null for ongoing)
/// - `season`: Season of release (Winter, Spring, Summer, Fall)
/// - `season_year`: Year of the release season
///
/// ## Statistics
/// - `average_score`: Average user rating (0-100)
/// - `mean_score`: Mean user rating (0-100)
/// - `popularity`: Number of users who have this in their lists
/// - `favourites`: Number of users who have favorited this anime
///
/// ## Visual Elements
/// - `cover_image`: Poster/cover art in multiple sizes
/// - `banner_image`: Wide banner image for backgrounds
///
/// ## External Links
/// - `site_url`: Direct link to this anime's AniList page
///
/// # Examples
///
/// ```rust
/// use anilist_sdk::AniListClient;
///
/// let client = AniListClient::new();
/// let anime = client.anime().get_by_id(16498).await?;
///
/// println!("Title: {}", anime.title.as_ref().unwrap().romaji);
/// println!("Episodes: {}", anime.episodes.unwrap_or(0));
/// println!("Score: {}/100", anime.average_score.unwrap_or(0));
///
/// if let Some(status) = &anime.status {
///     println!("Status: {:?}", status);
/// }
/// ```
///
/// # Note
///
/// Many fields may be `None` depending on the API endpoint used and the
/// completeness of data for the specific anime. Always check for `None`
/// values before using field data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anime {
    /// Unique identifier for this anime on AniList
    pub id: i32,
    /// Multi-language title information including romaji, english, and native titles
    pub title: Option<MediaTitle>,
    /// Synopsis or description of the anime (may contain HTML formatting)
    pub description: Option<String>,
    /// Format/type of the anime (TV series, movie, OVA, etc.)
    pub format: Option<MediaFormat>,
    /// Current airing/publication status
    pub status: Option<MediaStatus>,
    /// Date when the anime started airing
    pub start_date: Option<FuzzyDate>,
    /// Date when the anime finished airing (null for ongoing series)
    pub end_date: Option<FuzzyDate>,
    /// Season when the anime aired (Winter, Spring, Summer, Fall)
    pub season: Option<MediaSeason>,
    /// Year of the airing season
    pub season_year: Option<i32>,
    /// Total number of episodes (null for ongoing series)
    pub episodes: Option<i32>,
    /// Average episode duration in minutes
    pub duration: Option<i32>,
    /// List of genre tags associated with this anime
    pub genres: Option<Vec<String>>,
    /// Average user rating on a scale of 0-100
    pub average_score: Option<i32>,
    /// Mean user rating on a scale of 0-100
    pub mean_score: Option<i32>,
    /// Number of users who have this anime in their lists
    pub popularity: Option<i32>,
    /// Number of users who have favorited this anime
    pub favourites: Option<i32>,
    /// Official hashtag for social media
    pub hashtag: Option<String>,
    /// Country where the anime was produced
    pub country_of_origin: Option<String>,
    /// Whether the anime is marked as adult/mature content
    pub is_adult: Option<bool>,
    pub next_airing_episode: Option<AiringSchedule>,
    pub cover_image: Option<MediaCoverImage>,
    pub banner_image: Option<String>,
    pub studios: Option<StudioConnection>,
    pub source: Option<MediaSource>,
    pub trailer: Option<MediaTrailer>,
    pub updated_at: Option<i32>,
    pub site_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuzzyDate {
    pub year: Option<i32>,
    pub month: Option<i32>,
    pub day: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaFormat {
    Tv,
    TvShort,
    Movie,
    Special,
    Ova,
    Ona,
    Music,
    Manga,
    Novel,
    OneShot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaStatus {
    Finished,
    Releasing,
    NotYetReleased,
    Cancelled,
    Hiatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSeason {
    Winter,
    Spring,
    Summer,
    Fall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MediaSource {
    Original,
    Manga,
    LightNovel,
    VisualNovel,
    VideoGame,
    Other,
    Novel,
    Doujinshi,
    Anime,
    WebNovel,
    Liveaction,
    Game,
    Comic,
    MultimediaProject,
    PictureBook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AiringSchedule {
    pub id: i32,
    pub airing_at: i32,
    pub time_until_airing: i32,
    pub episode: i32,
    pub media_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaCoverImage {
    #[serde(rename = "extraLarge")]
    pub extra_large: Option<String>,
    pub large: Option<String>,
    pub medium: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTrailer {
    pub id: Option<String>,
    pub site: Option<String>,
    pub thumbnail: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioConnection {
    pub edges: Option<Vec<StudioEdge>>,
    pub nodes: Option<Vec<Studio>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioEdge {
    pub node: Option<Studio>,
    #[serde(rename = "isMain")]
    pub is_main: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Studio {
    pub id: i32,
    pub name: String,
    pub is_animation_studio: bool,
    pub site_url: Option<String>,
}
