use anilist_sdk::client::AniListClient;

mod test_utils;

#[tokio::test]
async fn test_get_popular_anime() {
    let client = AniListClient::new();

    let anime_list =
        crate::anime_api_call!(client, get_popular, 1, 5).expect("Failed to get popular anime");

    assert!(!anime_list.is_empty());
    assert!(anime_list.len() <= 5);

    // Check that all anime have required fields
    for anime in &anime_list {
        assert!(anime.id > 0);
        assert!(anime.title.is_some());
    }
}

#[tokio::test]
async fn test_get_trending_anime() {
    let client = AniListClient::new();

    let anime_list =
        crate::anime_api_call!(client, get_trending, 1, 3).expect("Failed to get trending anime");

    assert!(!anime_list.is_empty());
    assert!(anime_list.len() <= 3);
}

#[tokio::test]
async fn test_get_anime_by_id() {
    let client = AniListClient::new();

    // Using Attack on Titan's ID (16498)
    let anime =
        crate::anime_api_call!(client, get_by_id, 16498).expect("Failed to get anime by ID");

    assert_eq!(anime.id, 16498);
    assert!(anime.title.is_some());
}

#[tokio::test]
async fn test_search_anime() {
    let client = AniListClient::new();

    let anime_list =
        crate::anime_api_call!(client, search, "Naruto", 1, 5).expect("Failed to search anime");

    assert!(!anime_list.is_empty());

    // Check that results contain "Naruto" in some form
    let has_naruto = anime_list.iter().any(|anime| {
        if let Some(title) = &anime.title {
            title
                .romaji
                .as_ref()
                .is_some_and(|t| t.to_lowercase().contains("naruto"))
                || title
                    .english
                    .as_ref()
                    .is_some_and(|t| t.to_lowercase().contains("naruto"))
        } else {
            false
        }
    });
    assert!(has_naruto);
}

#[tokio::test]
async fn test_get_anime_by_season() {
    let client = AniListClient::new();

    let anime_list = crate::anime_api_call!(client, get_by_season, "FALL", 2023, 1, 5)
        .expect("Failed to get anime by season");

    assert!(!anime_list.is_empty());

    // Check that anime have the correct season and year
    for anime in &anime_list {
        if let Some(season_year) = anime.season_year {
            assert_eq!(season_year, 2023);
        }
    }
}

#[tokio::test]
async fn test_get_top_rated_anime() {
    let client = AniListClient::new();

    let anime_list =
        crate::anime_api_call!(client, get_top_rated, 1, 5).expect("Failed to get top rated anime");

    assert!(!anime_list.is_empty());

    // Check that anime have scores and are sorted by score
    let mut prev_score = 100;
    for anime in &anime_list {
        if let Some(score) = anime.average_score {
            assert!(score <= prev_score);
            prev_score = score;
        }
    }
}

#[tokio::test]
async fn test_get_airing_anime() {
    let client = AniListClient::new();

    let anime_list =
        crate::anime_api_call!(client, get_airing, 1, 5).expect("Failed to get airing anime");

    // Note: This might be empty if no anime are currently airing
    for anime in &anime_list {
        assert!(anime.id > 0);
        // Airing anime should have status RELEASING (though this might not always be set)
    }
}
