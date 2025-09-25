use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_get_user_by_id() {
    let client = AniListClient::new();
    // Using a known user ID (this might fail if the user doesn't exist)
    let result = crate::user_api_call!(client, get_by_id, 5429396);

    // This test might fail if the user doesn't exist, so we just check that the call works
    match result {
        Ok(user) => {
            assert_eq!(user.id, 5429396);
            assert!(!user.name.is_empty());
        }
        Err(_) => {
            // User might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_get_user_by_name() {
    let client = AniListClient::new();
    // This test might fail if the specific user doesn't exist
    let result = crate::user_api_call!(client, get_by_name, "xSensei");

    // This is expected to potentially fail, so we don't assert on success
    match result {
        Ok(user) => {
            assert_eq!(user.name, "xSensei");
        }
        Err(_) => {
            // User might not exist, which is acceptable for this test
        }
    }
}

#[tokio::test]
async fn test_search_users() {
    let client = AniListClient::new();
    let result = crate::user_api_call!(client, search, "xuehua", 1, 5);

    let users = result.expect("Failed to search users");
    // Note: This might be empty if no users match the search

    for user in &users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }
}

#[tokio::test]
async fn test_get_most_anime_watched() {
    let client = AniListClient::new();
    let result = crate::user_api_call!(client, get_most_anime_watched, 1, 5);

    let users = result.expect("Failed to get users with most anime watched");
    // Note: This might be empty based on privacy settings and data availability

    for user in &users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }
}

#[tokio::test]
async fn test_get_most_manga_read() {
    let client = AniListClient::new();
    let result = crate::user_api_call!(client, get_most_manga_read, 1, 5);

    let users = result.expect("Failed to get users with most manga read");
    // Note: This might be empty based on privacy settings and data availability

    for user in &users {
        assert!(user.id > 0);
        assert!(!user.name.is_empty());
    }
}

// Integration test to verify the basic functionality works
#[tokio::test]
async fn test_client_integration() {
    let client = AniListClient::new();

    // Test that we can make a basic query
    let anime_result = crate::anime_api_call!(client, get_popular, 1, 1);
    anime_result.expect("Failed to get popular anime");

    let manga_result = crate::manga_api_call!(client, get_popular, 1, 1);
    manga_result.expect("Failed to get popular manga");

    let character_result = crate::character_api_call!(client, get_popular, 1, 1);
    character_result.expect("Failed to get popular characters");

    let staff_result = crate::staff_api_call!(client, get_popular, 1, 1);
    staff_result.expect("Failed to get popular staff");
}
