pub mod anime;
pub mod character;
pub mod manga;
pub mod staff;
pub mod user;
pub mod media_list;
pub mod social;

// Re-export specific types to avoid ambiguity
pub use anime::{
    Anime, MediaTitle, FuzzyDate, MediaFormat, MediaStatus, MediaSeason, 
    MediaSource, AiringSchedule, MediaCoverImage, MediaTrailer, 
    StudioConnection, StudioEdge, Studio
};
pub use character::{Character, CharacterName, CharacterImage};
pub use manga::Manga;
pub use staff::{Staff, StaffName, StaffImage};
pub use user::{
    User, UserAvatar, UserOptions, NotificationOption, MediaListOptions, 
    MediaListTypeOptions, Favourites, UserStatistics, UserStatisticsType
};
pub use media_list::{MediaList, MediaListStatus, MediaListMedia};
pub use social::{
    Studio as SocialStudio, Thread, ThreadCategory, ThreadUser, ThreadComment,
    Review, ReviewUser, ReviewMedia, Recommendation, RecommendationUser, RecommendationMedia,
    Activity, TextActivity, ListActivity, MessageActivity, ActivityReply,
    Notification, NotificationUser, NotificationMedia, AiringSchedule as SocialAiringSchedule,
    AiringMedia, ActivityType, NotificationType, ReviewRating, RecommendationRating, MediaType
};
