use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub avatar: Option<UserAvatar>,
    pub banner_image: Option<String>,
    pub is_following: Option<bool>,
    pub is_follower: Option<bool>,
    pub is_blocked: Option<bool>,
    pub options: Option<UserOptions>,
    pub media_list_options: Option<MediaListOptions>,
    pub favourites: Option<Favourites>,
    pub statistics: Option<UserStatistics>,
    pub unread_notification_count: Option<i32>,
    pub site_url: Option<String>,
    pub donation_tier: Option<i32>,
    pub donation_badge: Option<String>,
    pub moderator_roles: Option<Vec<String>>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatar {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptions {
    pub title_language: Option<String>,
    pub display_adult_content: Option<bool>,
    pub airing_notifications: Option<bool>,
    pub profile_color: Option<String>,
    pub notification_options: Option<Vec<NotificationOption>>,
    pub timezone: Option<String>,
    pub activity_merge_time: Option<i32>,
    pub staff_name_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationOption {
    pub r#type: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListOptions {
    pub score_format: Option<String>,
    pub row_order: Option<String>,
    pub anime_list: Option<MediaListTypeOptions>,
    pub manga_list: Option<MediaListTypeOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListTypeOptions {
    pub section_order: Option<Vec<String>>,
    pub split_completed_section_by_format: Option<bool>,
    pub custom_lists: Option<Vec<String>>,
    pub advanced_scoring: Option<Vec<String>>,
    pub advanced_scoring_enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourites {
    pub anime: Option<MediaConnection>,
    pub manga: Option<MediaConnection>,
    pub characters: Option<CharacterConnection>,
    pub staff: Option<StaffConnection>,
    pub studios: Option<StudioConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConnection {
    pub nodes: Option<Vec<Media>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterConnection {
    pub nodes: Option<Vec<Character>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffConnection {
    pub nodes: Option<Vec<Staff>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudioConnection {
    pub nodes: Option<Vec<Studio>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: i32,
    pub title: Option<MediaTitle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTitle {
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: Option<CharacterName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterName {
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: Option<StaffName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffName {
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Studio {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatistics {
    pub anime: Option<UserStatisticsType>,
    pub manga: Option<UserStatisticsType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatisticsType {
    pub count: Option<i32>,
    pub mean_score: Option<f64>,
    pub standard_deviation: Option<f64>,
    pub minutes_watched: Option<i32>,
    pub episodes_watched: Option<i32>,
    pub chapters_read: Option<i32>,
    pub volumes_read: Option<i32>,
}
