use anilist_moe::client::AniListClient;

#[tokio::test]
async fn test_get_popular_characters() {
    let client = AniListClient::new();
    let result = client.character().get_popular(1, 5).await;
    
    assert!(result.is_ok());
    let characters = result.unwrap();
    assert!(!characters.is_empty());
    assert!(characters.len() <= 5);
    
    // Check that all characters have required fields
    for character in &characters {
        assert!(character.id > 0);
        assert!(character.name.is_some());
    }
}

#[tokio::test]
async fn test_get_character_by_id() {
    let client = AniListClient::new();
    // Using Lelouch vi Britannia's ID (417)
    let result = client.character().get_by_id(417).await;
    
    assert!(result.is_ok());
    let character = result.unwrap();
    assert_eq!(character.id, 417);
    assert!(character.name.is_some());
}

#[tokio::test]
async fn test_search_characters() {
    let client = AniListClient::new();
    let result = client.character().search("Luffy", 1, 5).await;
    
    assert!(result.is_ok());
    let characters = result.unwrap();
    assert!(!characters.is_empty());
    
    // Check that results contain "Luffy" in some form
    let has_luffy = characters.iter().any(|character| {
        if let Some(name) = &character.name {
            name.full.as_ref().map_or(false, |n| n.to_lowercase().contains("luffy")) ||
            name.first.as_ref().map_or(false, |n| n.to_lowercase().contains("luffy"))
        } else {
            false
        }
    });
    assert!(has_luffy);
}

#[tokio::test]
async fn test_get_characters_by_birthday() {
    let client = AniListClient::new();
    // Test with a specific date (March 15)
    let result = client.character().get_by_birthday(3, 15, 1, 10).await;
    
    assert!(result.is_ok());
    let characters = result.unwrap();
    // Note: This might be empty if no characters have this birthday
    
    for character in &characters {
        assert!(character.id > 0);
        if let Some(birth_date) = &character.date_of_birth {
            assert_eq!(birth_date.month, Some(3));
            assert_eq!(birth_date.day, Some(15));
        }
    }
}

#[tokio::test]
async fn test_get_most_favorited_characters() {
    let client = AniListClient::new();
    let result = client.character().get_most_favorited(1, 5).await;
    
    assert!(result.is_ok());
    let characters = result.unwrap();
    assert!(!characters.is_empty());
    
    // Check that characters are ordered by favorites (descending)
    let mut prev_favorites = i32::MAX;
    for character in &characters {
        assert!(character.id > 0);
        if let Some(favourites) = character.favourites {
            assert!(favourites <= prev_favorites);
            prev_favorites = favourites;
        }
    }
}
