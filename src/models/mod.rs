pub mod anime;
pub mod character;
pub mod manga;
pub mod staff;
pub mod user;

// Re-export specific types to avoid ambiguity
pub use anime::{
    AiringSchedule, Anime, FuzzyDate, MediaCoverImage, MediaFormat, MediaSeason, MediaSource,
    MediaStatus, MediaTitle, MediaTrailer, Studio, StudioConnection, StudioEdge,
};
pub use character::{Character, CharacterImage, CharacterName};
pub use manga::Manga;
pub use staff::{Staff, StaffImage, StaffName};
pub use user::{
    Favourites, MediaListOptions, MediaListTypeOptions, NotificationOption, User, UserAvatar,
    UserOptions, UserStatistics, UserStatisticsType,
};
