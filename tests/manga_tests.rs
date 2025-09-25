use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_get_popular_manga() {
    let client = AniListClient::new();
    let result = crate::manga_api_call!(client, get_popular, 1, 5);

    let manga_list = result.expect("Failed to get popular manga");
    assert!(!manga_list.is_empty());
    assert!(manga_list.len() <= 5);

    // Check that all manga have required fields
    for manga in &manga_list {
        assert!(manga.id > 0);
        assert!(manga.title.is_some());
    }
}

#[tokio::test]
async fn test_get_trending_manga() {
    let client = AniListClient::new();
    let result = crate::manga_api_call!(client, get_trending, 1, 3);

    let manga_list = result.expect("Failed to get trending manga");
    assert!(!manga_list.is_empty());
    assert!(manga_list.len() <= 3);
}

#[tokio::test]
async fn test_get_manga_by_id() {
    let client = AniListClient::new();
    // Using One Piece's ID (30013)
    let result = crate::manga_api_call!(client, get_by_id, 30013);

    let manga = result.expect("Failed to get manga by ID");
    assert_eq!(manga.id, 30013);
    assert!(manga.title.is_some());
}

#[tokio::test]
async fn test_search_manga() {
    let client = AniListClient::new();
    let result = crate::manga_api_call!(client, search, "One Piece", 1, 5);

    let manga_list = result.expect("Failed to search manga");
    assert!(!manga_list.is_empty());

    // Check that results contain "One Piece" in some form
    let has_one_piece = manga_list.iter().any(|manga| {
        if let Some(title) = &manga.title {
            title
                .romaji
                .as_ref()
                .is_some_and(|t| t.to_lowercase().contains("one piece"))
                || title
                    .english
                    .as_ref()
                    .is_some_and(|t| t.to_lowercase().contains("one piece"))
        } else {
            false
        }
    });
    assert!(has_one_piece);
}

#[tokio::test]
async fn test_get_top_rated_manga() {
    let client = AniListClient::new();
    let result = crate::manga_api_call!(client, get_top_rated, 1, 5);

    let manga_list = result.expect("Failed to get top rated manga");
    assert!(!manga_list.is_empty());

    // Check that manga have scores
    for manga in &manga_list {
        assert!(manga.id > 0);
        // Note: Not all manga may have scores, so we don't assert on that
    }
}

#[tokio::test]
async fn test_get_releasing_manga() {
    let client = AniListClient::new();
    let result = crate::manga_api_call!(client, get_releasing, 1, 5);

    let manga_list = result.expect("Failed to get releasing manga");
    // Note: This might be empty if no manga are currently releasing

    for manga in &manga_list {
        assert!(manga.id > 0);
    }
}

#[tokio::test]
async fn test_get_completed_manga() {
    let client = AniListClient::new();
    let result = crate::manga_api_call!(client, get_completed, 1, 5);

    let manga_list = result.expect("Failed to get completed manga");
    assert!(!manga_list.is_empty());

    for manga in &manga_list {
        assert!(manga.id > 0);
        assert!(manga.title.is_some());
    }
}
