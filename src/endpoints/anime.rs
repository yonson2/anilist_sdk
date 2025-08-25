use crate::client::AniListClient;
use crate::error::AniListError;
use crate::models::Anime;
use serde_json::json;
use std::collections::HashMap;

pub struct AnimeEndpoint {
    client: AniListClient,
}

impl AnimeEndpoint {
    pub(crate) fn new(client: AniListClient) -> Self {
        Self { client }
    }

    /// Get popular anime with pagination
    pub async fn get_popular(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, sort: POPULARITY_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        startDate {
                            year
                            month
                            day
                        }
                        endDate {
                            year
                            month
                            day
                        }
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        hashtag
                        countryOfOrigin
                        isAdult
                        nextAiringEpisode {
                            id
                            airingAt
                            timeUntilAiring
                            episode
                            mediaId
                        }
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        source
                        trailer {
                            id
                            site
                            thumbnail
                        }
                        updatedAt
                        siteUrl
                        studios {
                            nodes {
                                id
                                name
                                isAnimationStudio
                                siteUrl
                            }
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get trending anime
    pub async fn get_trending(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, sort: TRENDING_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        startDate {
                            year
                            month
                            day
                        }
                        endDate {
                            year
                            month
                            day
                        }
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get anime by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Anime, AniListError> {
        let query = r#"
            query ($id: Int) {
                Media(id: $id, type: ANIME) {
                    id
                    title {
                        romaji
                        english
                        native
                        userPreferred
                    }
                    description
                    format
                    status
                    startDate {
                        year
                        month
                        day
                    }
                    endDate {
                        year
                        month
                        day
                    }
                    season
                    seasonYear
                    episodes
                    duration
                    genres
                    averageScore
                    meanScore
                    popularity
                    favourites
                    hashtag
                    countryOfOrigin
                    isAdult
                    nextAiringEpisode {
                        id
                        airingAt
                        timeUntilAiring
                        episode
                        mediaId
                    }
                    coverImage {
                        extraLarge
                        large
                        medium
                        color
                    }
                    bannerImage
                    source
                    trailer {
                        id
                        site
                        thumbnail
                    }
                    updatedAt
                    siteUrl
                    studios {
                        nodes {
                            id
                            name
                            isAnimationStudio
                            siteUrl
                        }
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("id".to_string(), json!(id));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Media"].clone();
        let anime: Anime = serde_json::from_value(data)?;
        Ok(anime)
    }

    /// Search anime by title
    pub async fn search(&self, search: &str, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($search: String, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, search: $search) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("search".to_string(), json!(search));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get anime by season and year
    pub async fn get_by_season(&self, season: &str, year: i32, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($season: MediaSeason, $year: Int, $page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, season: $season, seasonYear: $year, sort: POPULARITY_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("season".to_string(), json!(season.to_uppercase()));
        variables.insert("year".to_string(), json!(year));
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get top rated anime
    pub async fn get_top_rated(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, sort: SCORE_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }

    /// Get currently airing anime
    pub async fn get_airing(&self, page: i32, per_page: i32) -> Result<Vec<Anime>, AniListError> {
        let query = r#"
            query ($page: Int, $perPage: Int) {
                Page(page: $page, perPage: $perPage) {
                    media(type: ANIME, status: RELEASING, sort: POPULARITY_DESC) {
                        id
                        title {
                            romaji
                            english
                            native
                            userPreferred
                        }
                        description
                        format
                        status
                        season
                        seasonYear
                        episodes
                        duration
                        genres
                        averageScore
                        meanScore
                        popularity
                        favourites
                        nextAiringEpisode {
                            id
                            airingAt
                            timeUntilAiring
                            episode
                            mediaId
                        }
                        coverImage {
                            extraLarge
                            large
                            medium
                            color
                        }
                        bannerImage
                        siteUrl
                    }
                }
            }
        "#;

        let mut variables = HashMap::new();
        variables.insert("page".to_string(), json!(page));
        variables.insert("perPage".to_string(), json!(per_page));

        let response = self.client.query(query, Some(variables)).await?;
        let data = response["data"]["Page"]["media"].clone();
        let anime_list: Vec<Anime> = serde_json::from_value(data)?;
        Ok(anime_list)
    }
}
