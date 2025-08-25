use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::user::User;
use serde_json::json;
use std::collections::HashMap;

pub struct UserEndpoint {
    client: AniListClient,
}

impl UserEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get user by ID
    pub async fn get_by_id(&self, id: i32) -> Result<User, AniListError> {
        let query = r#"
            query ($id: Int) {
                User(id: $id) {
                    id
                    name
                    about
                    avatar {
                        large
                        medium
                    }
                    bannerImage
                    isFollowing
                    isFollower
                    isBlocked
                    options {
                        titleLanguage
                        displayAdultContent
                        airingNotifications
                        profileColor
                        timezone
                        activityMergeTime
                        staffNameLanguage
                    }
                    mediaListOptions {
                        scoreFormat
                        rowOrder
                        animeList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                        mangaList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                    }
                    favourites {
                        anime {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        manga {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        characters {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        staff {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        studios {
                            nodes {
                                id
                                name
                            }
                        }
                    }
                    statistics {
                        anime {
                            count
                            meanScore
                            standardDeviation
                            minutesWatched
                            episodesWatched
                        }
                        manga {
                            count
                            meanScore
                            standardDeviation
                            chaptersRead
                            volumesRead
                        }
                    }
                    unreadNotificationCount
                    siteUrl
                    donatorTier
                    donatorBadge
                    moderatorRoles
                    createdAt
                    updatedAt
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["User"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Get user by name
    pub async fn get_by_name(&self, name: &str) -> Result<User, AniListError> {
        let query = r#"
            query ($name: String) {
                User(name: $name) {
                    id
                    name
                    about
                    avatar {
                        large
                        medium
                    }
                    bannerImage
                    isFollowing
                    isFollower
                    isBlocked
                    options {
                        titleLanguage
                        displayAdultContent
                        airingNotifications
                        profileColor
                        timezone
                        activityMergeTime
                        staffNameLanguage
                    }
                    mediaListOptions {
                        scoreFormat
                        rowOrder
                        animeList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                        mangaList {
                            sectionOrder
                            splitCompletedSectionByFormat
                            customLists
                            advancedScoring
                            advancedScoringEnabled
                        }
                    }
                    favourites {
                        anime {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        manga {
                            nodes {
                                id
                                title {
                                    userPreferred
                                }
                            }
                        }
                        characters {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        staff {
                            nodes {
                                id
                                name {
                                    userPreferred
                                }
                            }
                        }
                        studios {
                            nodes {
                                id
                                name
                            }
                        }
                    }
                    statistics {
                        anime {
                            count
                            meanScore
                            standardDeviation
                            minutesWatched
                            episodesWatched
                        }
                        manga {
                            count
                            meanScore
                            standardDeviation
                            chaptersRead
                            volumesRead
                        }
                    }
                    unreadNotificationCount
                    siteUrl
                    donatorTier
                    donatorBadge
                    moderatorRoles
                    createdAt
                    updatedAt
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("name".to_string(), json!(name));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["User"].clone();
        let user: User = serde_json::from_value(data)?;
        Ok(user)
    }

    /// Search users by name
    pub async fn search(&self, search: &str, page: i32, per_page: i32) -> Result<Vec<User>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    users(search: $search) {
                        id
                        name
                        about
                        avatar {
                            large
                            medium
                        }
                        bannerImage
                        statistics {
                            anime {
                                count
                                meanScore
                                minutesWatched
                                episodesWatched
                            }
                            manga {
                                count
                                meanScore
                                chaptersRead
                                volumesRead
                            }
                        }
                        siteUrl
                        donatorTier
                        createdAt
                        updatedAt
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Get users with most anime watched
    pub async fn get_most_anime_watched(&self, page: i32, per_page: i32) -> Result<Vec<User>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    users(sort: WATCHED_TIME_DESC) {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                        statistics {
                            anime {
                                count
                                meanScore
                                minutesWatched
                                episodesWatched
                            }
                            manga {
                                count
                                meanScore
                                chaptersRead
                                volumesRead
                            }
                        }
                        siteUrl
                        donatorTier
                        createdAt
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }

    /// Get users with most manga read
    pub async fn get_most_manga_read(&self, page: i32, per_page: i32) -> Result<Vec<User>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    users(sort: CHAPTERS_READ_DESC) {
                        id
                        name
                        avatar {
                            large
                            medium
                        }
                        statistics {
                            anime {
                                count
                                meanScore
                                minutesWatched
                                episodesWatched
                            }
                            manga {
                                count
                                meanScore
                                chaptersRead
                                volumesRead
                            }
                        }
                        siteUrl
                        donatorTier
                        createdAt
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["users"].clone();
        let users: Vec<User> = serde_json::from_value(data)?;
        Ok(users)
    }
}
