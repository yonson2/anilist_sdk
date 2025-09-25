use anilist_sdk::client::AniListClient;
mod test_utils;

#[tokio::test]
async fn test_comprehensive_integration() {
    let client = AniListClient::new();

    // Test anime endpoints
    println!("Testing anime endpoints...");

    let popular_anime =
        crate::anime_api_call!(client, get_popular, 1, 3).expect("Failed to get popular anime");
    assert!(!popular_anime.is_empty());
    println!("✓ Popular anime: Found {} entries", popular_anime.len());

    let trending_anime =
        crate::anime_api_call!(client, get_trending, 1, 3).expect("Failed to get trending anime");
    assert!(!trending_anime.is_empty());
    println!("✓ Trending anime: Found {} entries", trending_anime.len());

    // Get a specific anime by ID
    if let Some(first_anime) = popular_anime.first() {
        let anime_id = first_anime.id;
        let anime_by_id =
            crate::anime_api_call!(client, get_by_id, anime_id).expect("Failed to get anime by ID");
        assert_eq!(anime_by_id.id, anime_id);
        println!("✓ Anime by ID: Successfully retrieved anime {}", anime_id);
    }

    // Test manga endpoints
    println!("Testing manga endpoints...");

    let popular_manga =
        crate::manga_api_call!(client, get_popular, 1, 3).expect("Failed to get popular manga");
    assert!(!popular_manga.is_empty());
    println!("✓ Popular manga: Found {} entries", popular_manga.len());

    let trending_manga =
        crate::manga_api_call!(client, get_trending, 1, 3).expect("Failed to get trending manga");
    assert!(!trending_manga.is_empty());
    println!("✓ Trending manga: Found {} entries", trending_manga.len());

    // Test character endpoints
    println!("Testing character endpoints...");

    let popular_characters = crate::character_api_call!(client, get_popular, 1, 3)
        .expect("Failed to get popular characters");
    assert!(!popular_characters.is_empty());
    println!(
        "✓ Popular characters: Found {} entries",
        popular_characters.len()
    );

    // Test staff endpoints
    println!("Testing staff endpoints...");

    let popular_staff =
        crate::staff_api_call!(client, get_popular, 1, 3).expect("Failed to get popular staff");
    assert!(!popular_staff.is_empty());
    println!("✓ Popular staff: Found {} entries", popular_staff.len());

    // Test search functionality
    println!("Testing search functionality...");

    let anime_search = crate::anime_api_call!(client, search, "Attack on Titan", 1, 2)
        .expect("Failed to search anime");
    assert!(!anime_search.is_empty());
    println!(
        "✓ Anime search: Found {} entries for 'Attack on Titan'",
        anime_search.len()
    );

    let character_search = crate::character_api_call!(client, search, "Eren", 1, 2)
        .expect("Failed to search characters");
    assert!(!character_search.is_empty());
    println!(
        "✓ Character search: Found {} entries for 'Eren'",
        character_search.len()
    );

    println!("All integration tests passed! 🎉");
}

#[tokio::test]
async fn test_error_handling() {
    let client = AniListClient::new();

    // Test with invalid ID (should not panic, but might return an error)
    let result = crate::anime_api_call!(client, get_by_id, -1);

    // The API might return an error or empty result for invalid IDs
    // We just want to make sure it doesn't panic
    match result {
        Ok(_) => println!("API returned a result for invalid ID (unexpected but not an error)"),
        Err(e) => println!("API correctly returned an error for invalid ID: {:?}", e),
    }
}

#[tokio::test]
async fn test_pagination() {
    let client = AniListClient::new();

    // Test that pagination works correctly
    let page1 = crate::anime_api_call!(client, get_popular, 1, 5).expect("Failed to get page 1");
    let page2 = crate::anime_api_call!(client, get_popular, 2, 5).expect("Failed to get page 2");

    assert!(!page1.is_empty());
    assert!(!page2.is_empty());

    // Check that pages contain different content
    let page1_ids: Vec<i32> = page1.iter().map(|a| a.id).collect();
    let page2_ids: Vec<i32> = page2.iter().map(|a| a.id).collect();

    let has_different_content = page1_ids.iter().any(|id| !page2_ids.contains(id));
    assert!(
        has_different_content,
        "Pages should contain different content"
    );

    println!("✓ Pagination works correctly");
}
