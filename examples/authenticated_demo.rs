use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example of using unauthenticated client
    println!("=== Unauthenticated Client Example ===");
    let client = AniListClient::new();
    
    let popular_anime = client.anime().get_popular(1, 3).await?;
    println!("Popular anime (first 3):");
    for anime in popular_anime {
        if let Some(title) = &anime.title {
            println!("  - {} (ID: {})", 
                title.user_preferred.as_ref()
                    .or(title.english.as_ref())
                    .or(title.romaji.as_ref())
                    .unwrap_or(&"Unknown".to_string()), 
                anime.id
            );
        }
    }
    
    // Example of using authenticated client
    println!("\n=== Authenticated Client Example ===");
    
    // In a real application, you would get this token from AniList OAuth flow
    // For now, we'll show how to create the client (but not use it since we don't have a real token)
    let token = "your_access_token_here".to_string();
    let _authenticated_client = AniListClient::with_token(token);
    
    println!("Authenticated client created with token support.");
    println!("To use authenticated features:");
    println!("1. Implement AniList OAuth flow to get an access token");
    println!("2. Create client with: AniListClient::with_token(your_token)");
    println!("3. Use methods like get_current_user() that require authentication");
    
    // Example of what you could do with an authenticated client:
    /*
    match authenticated_client.user().get_current_user().await {
        Ok(user) => {
            println!("Current user: {}", user.name);
            if let Some(stats) = user.statistics {
                if let Some(anime_stats) = stats.anime {
                    println!("Anime watched: {} episodes", anime_stats.episodes_watched.unwrap_or(0));
                }
            }
        }
        Err(e) => println!("Failed to get current user: {}", e),
    }
    */
    
    // Demonstrate other client capabilities
    println!("\n=== Search Example ===");
    let search_results = client.anime().search("Attack on Titan", 1, 2).await?;
    println!("Search results for 'Attack on Titan':");
    for anime in search_results {
        if let Some(title) = &anime.title {
            println!("  - {} (ID: {})", 
                title.user_preferred.as_ref()
                    .or(title.english.as_ref())
                    .or(title.romaji.as_ref())
                    .unwrap_or(&"Unknown".to_string()), 
                anime.id
            );
        }
    }
    
    println!("\n=== Character Example ===");
    let popular_characters = client.character().get_popular(1, 3).await?;
    println!("Popular characters:");
    for character in popular_characters {
        if let Some(name) = &character.name {
            println!("  - {} (ID: {})", 
                name.user_preferred.as_ref()
                    .or(name.full.as_ref())
                    .unwrap_or(&"Unknown".to_string()), 
                character.id
            );
        }
    }
    
    Ok(())
}
