//! # GraphQL Queries
//!
//! This module contains all GraphQL queries used by the AniList API wrapper.
//! Queries are organized by endpoint category and loaded from separate .graphql files.

/// Anime-related GraphQL queries
pub mod anime {
    /// Get popular anime query
    pub const GET_POPULAR: &str = include_str!("anime/get_popular.graphql");

    /// Get trending anime query
    pub const GET_TRENDING: &str = include_str!("anime/get_trending.graphql");

    /// Search anime query
    pub const SEARCH: &str = include_str!("anime/search.graphql");

    /// Get anime by ID query
    pub const GET_BY_ID: &str = include_str!("anime/get_by_id.graphql");

    /// Get anime by season query
    pub const GET_BY_SEASON: &str = include_str!("anime/get_by_season.graphql");

    /// Get top rated anime query
    pub const GET_TOP_RATED: &str = include_str!("anime/get_top_rated.graphql");

    /// Get currently airing anime query
    pub const GET_AIRING: &str = include_str!("anime/get_airing.graphql");
}

/// User-related GraphQL queries
pub mod user {
    /// Get current user (Viewer) query
    pub const GET_CURRENT_USER: &str = include_str!("user/get_current_user.graphql");

    /// Get current user's anime list query
    pub const GET_CURRENT_USER_ANIME_LIST: &str =
        include_str!("user/get_current_user_anime_list.graphql");

    /// Get user by ID query
    pub const GET_BY_ID: &str = include_str!("user/get_by_id.graphql");

    /// Get user by name query
    pub const GET_BY_NAME: &str = include_str!("user/get_by_name.graphql");

    /// Search users query
    pub const SEARCH: &str = include_str!("user/search.graphql");

    /// Get users with most anime watched query
    pub const GET_MOST_ANIME_WATCHED: &str = include_str!("user/get_most_anime_watched.graphql");

    /// Get users with most manga read query
    pub const GET_MOST_MANGA_READ: &str = include_str!("user/get_most_manga_read.graphql");

    /// Toggle follow/unfollow user mutation
    pub const TOGGLE_FOLLOW: &str = include_str!("user/toggle_follow.graphql");

    /// Toggle favorite anime/manga mutation
    pub const TOGGLE_FAVORITE: &str = include_str!("user/toggle_favorite.graphql");

    /// Update media list progress mutation
    pub const UPDATE_MEDIA_LIST_PROGRESS: &str =
        include_str!("user/update_media_list_progress.graphql");

    /// Update media list status mutation
    pub const UPDATE_MEDIA_LIST_STATUS: &str =
        include_str!("user/update_media_list_status.graphql");
}

/// Manga-related GraphQL queries
pub mod manga {
    /// Get popular manga query
    pub const GET_POPULAR: &str = include_str!("manga/get_popular.graphql");

    /// Get trending manga query
    pub const GET_TRENDING: &str = include_str!("manga/get_trending.graphql");

    /// Get manga by ID query
    pub const GET_BY_ID: &str = include_str!("manga/get_by_id.graphql");

    /// Search manga query
    pub const SEARCH: &str = include_str!("manga/search.graphql");

    /// Get top rated manga query
    pub const GET_TOP_RATED: &str = include_str!("manga/get_top_rated.graphql");

    /// Get currently releasing manga query
    pub const GET_RELEASING: &str = include_str!("manga/get_releasing.graphql");

    /// Get completed manga query
    pub const GET_COMPLETED: &str = include_str!("manga/get_completed.graphql");
}

/// Character-related GraphQL queries
pub mod character {
    /// Get popular characters query
    pub const GET_POPULAR: &str = include_str!("character/get_popular.graphql");

    /// Get character by ID query
    pub const GET_BY_ID: &str = include_str!("character/get_by_id.graphql");

    /// Search characters query
    pub const SEARCH: &str = include_str!("character/search.graphql");

    /// Get characters with today's birthday query
    pub const GET_TODAY_BIRTHDAY: &str = include_str!("character/get_today_birthday.graphql");

    /// Get most favorited characters query
    pub const GET_MOST_FAVORITED: &str = include_str!("character/get_most_favorited.graphql");
}

/// Staff-related GraphQL queries
pub mod staff {
    /// Get popular staff query
    pub const GET_POPULAR: &str = include_str!("staff/get_popular.graphql");

    /// Get staff by ID query
    pub const GET_BY_ID: &str = include_str!("staff/get_by_id.graphql");

    /// Search staff query
    pub const SEARCH: &str = include_str!("staff/search.graphql");

    /// Get staff with today's birthday query
    pub const GET_TODAY_BIRTHDAY: &str = include_str!("staff/get_today_birthday.graphql");

    /// Get most favorited staff query
    pub const GET_MOST_FAVORITED: &str = include_str!("staff/get_most_favorited.graphql");
}

/// Studio-related GraphQL queries
pub mod studio {
    /// Get popular studios query
    pub const GET_POPULAR: &str = include_str!("studio/get_popular.graphql");

    /// Get studio by ID query
    pub const GET_BY_ID: &str = include_str!("studio/get_by_id.graphql");

    /// Search studios query
    pub const SEARCH: &str = include_str!("studio/search.graphql");

    /// Get most favorited studios query
    pub const GET_MOST_FAVORITED: &str = include_str!("studio/get_most_favorited.graphql");

    /// Toggle favorite studio mutation
    pub const TOGGLE_FAVORITE: &str = include_str!("studio/toggle_favorite.graphql");
}

/// Activity-related GraphQL queries
pub mod activity {
    /// Get recent activities query
    pub const GET_RECENT_ACTIVITIES: &str = include_str!("activity/get_recent_activities.graphql");

    /// Get following activities query
    pub const GET_FOLLOWING_ACTIVITIES: &str =
        include_str!("activity/get_following_activities.graphql");

    /// Get user activities query
    pub const GET_USER_ACTIVITIES: &str = include_str!("activity/get_user_activities.graphql");

    /// Get text activities query
    pub const GET_TEXT_ACTIVITIES: &str = include_str!("activity/get_text_activities.graphql");

    /// Get activity by ID query
    pub const GET_ACTIVITY_BY_ID: &str = include_str!("activity/get_activity_by_id.graphql");

    /// Get activity replies query
    pub const GET_ACTIVITY_REPLIES: &str = include_str!("activity/get_activity_replies.graphql");

    /// Create text activity mutation
    pub const CREATE_TEXT_ACTIVITY: &str = include_str!("activity/create_text_activity.graphql");

    /// Toggle activity reply like mutation
    pub const TOGGLE_ACTIVITY_REPLY_LIKE: &str =
        include_str!("activity/toggle_activity_reply_like.graphql");

    /// Delete activity mutation
    pub const DELETE_ACTIVITY: &str = include_str!("activity/delete_activity.graphql");

    /// Toggle like on activity mutation
    pub const TOGGLE_LIKE: &str = include_str!("activity/toggle_like.graphql");

    /// Reply to activity mutation
    pub const REPLY_TO_ACTIVITY: &str = include_str!("activity/reply_to_activity.graphql");
}

/// Forum-related GraphQL queries
pub mod forum {
    /// Get recent threads query
    pub const GET_RECENT_THREADS: &str = include_str!("forum/get_recent_threads.graphql");

    /// Get thread by ID query
    pub const GET_THREAD_BY_ID: &str = include_str!("forum/get_thread_by_id.graphql");

    /// Search threads query
    pub const SEARCH_THREADS: &str = include_str!("forum/search_threads.graphql");

    /// Get thread comments query
    pub const GET_THREAD_COMMENTS: &str = include_str!("forum/get_thread_comments.graphql");

    /// Create thread mutation
    pub const CREATE_THREAD: &str = include_str!("forum/create_thread.graphql");

    /// Toggle thread like mutation
    pub const TOGGLE_THREAD_LIKE: &str = include_str!("forum/toggle_thread_like.graphql");

    /// Comment on thread mutation
    pub const COMMENT_ON_THREAD: &str = include_str!("forum/comment_on_thread.graphql");

    /// Like thread comment mutation
    pub const LIKE_THREAD_COMMENT: &str = include_str!("forum/like_thread_comment.graphql");
}

/// Recommendation-related GraphQL queries
pub mod recommendation {
    /// Get recent recommendations query
    pub const GET_RECENT_RECOMMENDATIONS: &str =
        include_str!("recommendation/get_recent_recommendations.graphql");

    /// Get recommendations for media query
    pub const GET_RECOMMENDATIONS_FOR_MEDIA: &str =
        include_str!("recommendation/get_recommendations_for_media.graphql");

    /// Get top rated recommendations query
    pub const GET_TOP_RATED_RECOMMENDATIONS: &str =
        include_str!("recommendation/get_top_rated_recommendations.graphql");

    /// Get recommendation by ID query
    pub const GET_RECOMMENDATION_BY_ID: &str =
        include_str!("recommendation/get_recommendation_by_id.graphql");

    /// Save recommendation mutation
    pub const SAVE_RECOMMENDATION: &str =
        include_str!("recommendation/save_recommendation.graphql");

    /// Rate recommendation mutation
    pub const RATE_RECOMMENDATION: &str =
        include_str!("recommendation/rate_recommendation.graphql");
}

/// Notification-related GraphQL queries
pub mod notification {
    /// Get notifications query
    pub const GET_NOTIFICATIONS: &str = include_str!("notification/get_notifications.graphql");

    /// Get unread count query
    pub const GET_UNREAD_COUNT: &str = include_str!("notification/get_unread_count.graphql");

    /// Get notifications by type query
    pub const GET_NOTIFICATIONS_BY_TYPE: &str =
        include_str!("notification/get_notifications_by_type.graphql");

    /// Mark notifications as read mutation
    pub const MARK_NOTIFICATIONS_AS_READ: &str =
        include_str!("notification/mark_notifications_as_read.graphql");
}

/// Review-related GraphQL queries
pub mod review {
    /// Get recent reviews query
    pub const GET_RECENT_REVIEWS: &str = include_str!("review/get_recent_reviews.graphql");

    /// Get reviews for media query
    pub const GET_REVIEWS_FOR_MEDIA: &str = include_str!("review/get_reviews_for_media.graphql");

    /// Get reviews by user query
    pub const GET_REVIEWS_BY_USER: &str = include_str!("review/get_reviews_by_user.graphql");

    /// Get review by ID query
    pub const GET_REVIEW_BY_ID: &str = include_str!("review/get_review_by_id.graphql");

    /// Get top rated reviews query
    pub const GET_TOP_RATED_REVIEWS: &str = include_str!("review/get_top_rated_reviews.graphql");

    /// Save review mutation
    pub const SAVE_REVIEW: &str = include_str!("review/save_review.graphql");

    /// Rate review mutation
    pub const RATE_REVIEW: &str = include_str!("review/rate_review.graphql");

    /// Delete review mutation
    pub const DELETE_REVIEW: &str = include_str!("review/delete_review.graphql");
}

/// Airing-related GraphQL queries
pub mod airing {
    /// Get upcoming episodes query
    pub const GET_UPCOMING_EPISODES: &str = include_str!("airing/get_upcoming_episodes.graphql");

    /// Get today's episodes query
    pub const GET_TODAY_EPISODES: &str = include_str!("airing/get_today_episodes.graphql");

    /// Get recently aired episodes query
    pub const GET_RECENTLY_AIRED: &str = include_str!("airing/get_recently_aired.graphql");

    /// Get schedule for media query
    pub const GET_SCHEDULE_FOR_MEDIA: &str = include_str!("airing/get_schedule_for_media.graphql");

    /// Get schedule by ID query
    pub const GET_SCHEDULE_BY_ID: &str = include_str!("airing/get_schedule_by_id.graphql");

    /// Get episodes in range query
    pub const GET_EPISODES_IN_RANGE: &str = include_str!("airing/get_episodes_in_range.graphql");

    /// Get next episode query
    pub const GET_NEXT_EPISODE: &str = include_str!("airing/get_next_episode.graphql");
}
