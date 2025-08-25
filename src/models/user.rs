use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub avatar: Option<UserAvatar>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    #[serde(rename = "isFollowing")]
    pub is_following: Option<bool>,
    #[serde(rename = "isFollower")]
    pub is_follower: Option<bool>,
    #[serde(rename = "isBlocked")]
    pub is_blocked: Option<bool>,
    pub options: Option<UserOptions>,
    #[serde(rename = "mediaListOptions")]
    pub media_list_options: Option<MediaListOptions>,
    pub favourites: Option<Favourites>,
    pub statistics: Option<UserStatistics>,
    #[serde(rename = "unreadNotificationCount")]
    pub unread_notification_count: Option<i32>,
    #[serde(rename = "siteUrl")]
    pub site_url: Option<String>,
    #[serde(rename = "donatorTier")]
    pub donation_tier: Option<i32>,
    #[serde(rename = "donationBadge")]
    pub donation_badge: Option<String>,
    #[serde(rename = "moderatorRoles")]
    pub moderator_roles: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<i32>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAvatar {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOptions {
    #[serde(rename = "titleLanguage")]
    pub title_language: Option<String>,
    #[serde(rename = "displayAdultContent")]
    pub display_adult_content: Option<bool>,
    #[serde(rename = "airingNotifications")]
    pub airing_notifications: Option<bool>,
    #[serde(rename = "profileColor")]
    pub profile_color: Option<String>,
    #[serde(rename = "notificationOptions")]
    pub notification_options: Option<Vec<NotificationOption>>,
    pub timezone: Option<String>,
    #[serde(rename = "activityMergeTime")]
    pub activity_merge_time: Option<i32>,
    #[serde(rename = "staffNameLanguage")]
    pub staff_name_language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationOption {
    pub r#type: Option<String>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListOptions {
    #[serde(rename = "scoreFormat")]
    pub score_format: Option<String>,
    #[serde(rename = "rowOrder")]
    pub row_order: Option<String>,
    #[serde(rename = "animeList")]
    pub anime_list: Option<MediaListTypeOptions>,
    #[serde(rename = "mangaList")]
    pub manga_list: Option<MediaListTypeOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListTypeOptions {
    #[serde(rename = "sectionOrder")]
    pub section_order: Option<Vec<String>>,
    #[serde(rename = "splitCompletedSectionByFormat")]
    pub split_completed_section_by_format: Option<bool>,
    #[serde(rename = "customLists")]
    pub custom_lists: Option<Vec<String>>,
    #[serde(rename = "advancedScoring")]
    pub advanced_scoring: Option<Vec<String>>,
    #[serde(rename = "advancedScoringEnabled")]
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
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: Option<CharacterName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterName {
    #[serde(rename = "userPreferred")]
    pub user_preferred: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: Option<StaffName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffName {
    #[serde(rename = "userPreferred")]
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
    #[serde(rename = "meanScore")]
    pub mean_score: Option<f64>,
    #[serde(rename = "standardDeviation")]
    pub standard_deviation: Option<f64>,
    #[serde(rename = "minutesWatched")]
    pub minutes_watched: Option<i32>,
    #[serde(rename = "episodesWatched")]
    pub episodes_watched: Option<i32>,
    #[serde(rename = "chaptersRead")]
    pub chapters_read: Option<i32>,
    #[serde(rename = "volumesRead")]
    pub volumes_read: Option<i32>,
}
