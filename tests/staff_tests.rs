use anilist_moe::client::AniListClient;

#[tokio::test]
async fn test_get_popular_staff() {
    let client = AniListClient::new();
    let result = client.staff().get_popular(1, 5).await;
    
    assert!(result.is_ok());
    let staff_list = result.unwrap();
    assert!(!staff_list.is_empty());
    assert!(staff_list.len() <= 5);
    
    // Check that all staff have required fields
    for staff in &staff_list {
        assert!(staff.id > 0);
        assert!(staff.name.is_some());
    }
}

#[tokio::test]
async fn test_get_staff_by_id() {
    let client = AniListClient::new();
    // Using Hayao Miyazaki's ID (1870)
    let result = client.staff().get_by_id(1870).await;
    
    assert!(result.is_ok());
    let staff = result.unwrap();
    assert_eq!(staff.id, 1870);
    assert!(staff.name.is_some());
}

#[tokio::test]
async fn test_search_staff() {
    let client = AniListClient::new();
    let result = client.staff().search("Miyazaki", 1, 5).await;
    
    assert!(result.is_ok());
    let staff_list = result.unwrap();
    assert!(!staff_list.is_empty());
    
    // Check that results contain "Miyazaki" in some form
    let has_miyazaki = staff_list.iter().any(|staff| {
        if let Some(name) = &staff.name {
            name.full.as_ref().map_or(false, |n| n.to_lowercase().contains("miyazaki")) ||
            name.last.as_ref().map_or(false, |n| n.to_lowercase().contains("miyazaki"))
        } else {
            false
        }
    });
    assert!(has_miyazaki);
}

#[tokio::test]
async fn test_get_staff_by_birthday() {
    let client = AniListClient::new();
    // Test with a specific date (January 5)
    let result = client.staff().get_by_birthday(1, 5, 1, 10).await;
    
    assert!(result.is_ok());
    let staff_list = result.unwrap();
    // Note: This might be empty if no staff have this birthday
    
    for staff in &staff_list {
        assert!(staff.id > 0);
        if let Some(birth_date) = &staff.date_of_birth {
            assert_eq!(birth_date.month, Some(1));
            assert_eq!(birth_date.day, Some(5));
        }
    }
}

#[tokio::test]
async fn test_get_most_favorited_staff() {
    let client = AniListClient::new();
    let result = client.staff().get_most_favorited(1, 5).await;
    
    assert!(result.is_ok());
    let staff_list = result.unwrap();
    assert!(!staff_list.is_empty());
    
    // Check that staff are ordered by favorites (descending)
    let mut prev_favorites = i32::MAX;
    for staff in &staff_list {
        assert!(staff.id > 0);
        if let Some(favourites) = staff.favourites {
            assert!(favourites <= prev_favorites);
            prev_favorites = favourites;
        }
    }
}
