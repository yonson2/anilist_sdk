pub mod anime;
pub mod character;
pub mod manga;
pub mod staff;
pub mod user;

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
