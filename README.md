# AniList API Wrapper

A comprehensive, modular Rust wrapper for the AniList GraphQL API.

## Features

- **Modular Design**: Separate endpoints for anime, manga, characters, staff, and users
- **Async/Await Support**: Built with Tokio for asynchronous operations
- **Type Safety**: Strongly typed responses with Serde serialization
- **Comprehensive Coverage**: Supports popular, trending, search, and detailed queries
- **Error Handling**: Proper error types for different failure scenarios
- **Pagination**: Built-in support for paginated results
- **Tested**: Comprehensive test suite covering all endpoints

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
anilist-moe = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Usage Examples

### Basic Usage

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Get popular anime
    let popular_anime = client.anime().get_popular(1, 10).await?;
    println!("Popular anime: {:#?}", popular_anime);
    
    Ok(())
}
```

### Anime Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Get popular anime
    let popular = client.anime().get_popular(1, 5).await?;
    
    // Get trending anime
    let trending = client.anime().get_trending(1, 5).await?;
    
    // Get anime by ID
    let anime = client.anime().get_by_id(16498).await?; // Attack on Titan
    
    // Search anime
    let search_results = client.anime().search("Naruto", 1, 10).await?;
    
    // Get anime by season
    let fall_2023 = client.anime().get_by_season("FALL", 2023, 1, 10).await?;
    
    // Get top rated anime
    let top_rated = client.anime().get_top_rated(1, 10).await?;
    
    // Get currently airing anime
    let airing = client.anime().get_airing(1, 10).await?;
    
    Ok(())
}
```

### Manga Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Get popular manga
    let popular = client.manga().get_popular(1, 5).await?;
    
    // Get trending manga
    let trending = client.manga().get_trending(1, 5).await?;
    
    // Get manga by ID
    let manga = client.manga().get_by_id(30013).await?; // One Piece
    
    // Search manga
    let search_results = client.manga().search("One Piece", 1, 10).await?;
    
    // Get top rated manga
    let top_rated = client.manga().get_top_rated(1, 10).await?;
    
    // Get currently releasing manga
    let releasing = client.manga().get_releasing(1, 10).await?;
    
    // Get completed manga
    let completed = client.manga().get_completed(1, 10).await?;
    
    Ok(())
}
```

### Character Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Get popular characters
    let popular = client.character().get_popular(1, 10).await?;
    
    // Get character by ID
    let character = client.character().get_by_id(417).await?; // Lelouch
    
    // Search characters
    let search_results = client.character().search("Luffy", 1, 10).await?;
    
    // Get characters by birthday
    let birthday_chars = client.character().get_by_birthday(3, 15, 1, 10).await?;
    
    // Get most favorited characters
    let most_favorited = client.character().get_most_favorited(1, 10).await?;
    
    Ok(())
}
```

### Staff Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Get popular staff
    let popular = client.staff().get_popular(1, 10).await?;
    
    // Get staff by ID
    let staff = client.staff().get_by_id(1870).await?; // Hayao Miyazaki
    
    // Search staff
    let search_results = client.staff().search("Miyazaki", 1, 10).await?;
    
    // Get staff by birthday
    let birthday_staff = client.staff().get_by_birthday(1, 5, 1, 10).await?;
    
    // Get most favorited staff
    let most_favorited = client.staff().get_most_favorited(1, 10).await?;
    
    Ok(())
}
```

### User Operations

```rust
use anilist_moe::client::AniListClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AniListClient::new();
    
    // Get user by ID
    let user = client.user().get_by_id(1).await?;
    
    // Get user by name
    let user = client.user().get_by_name("username").await?;
    
    // Search users
    let search_results = client.user().search("test", 1, 10).await?;
    
    // Get users with most anime watched
    let anime_watchers = client.user().get_most_anime_watched(1, 10).await?;
    
    // Get users with most manga read
    let manga_readers = client.user().get_most_manga_read(1, 10).await?;
    
    Ok(())
}
```

## Error Handling

The library provides comprehensive error handling:

```rust
use anilist_moe::{client::AniListClient, error::AniListError};

#[tokio::main]
async fn main() {
    let client = AniListClient::new();
    
    match client.anime().get_by_id(999999).await {
        Ok(anime) => println!("Found anime: {:?}", anime),
        Err(AniListError::Network(e)) => println!("Network error: {}", e),
        Err(AniListError::GraphQL { message }) => println!("GraphQL error: {}", message),
        Err(AniListError::Json(e)) => println!("JSON parsing error: {}", e),
        Err(AniListError::RateLimit) => println!("Rate limited"),
        Err(AniListError::NotFound) => println!("Not found"),
    }
}
```

## Data Models

The library includes comprehensive data models for all AniList entities:

- **Anime**: Complete anime information including titles, descriptions, episodes, genres, etc.
- **Manga**: Complete manga information including chapters, volumes, status, etc.
- **Character**: Character details including names, images, descriptions, birthdays, etc.
- **Staff**: Staff information including names, roles, occupations, etc.
- **User**: User profiles including statistics, favorites, and preferences.

## Testing

Run the test suite:

```bash
cargo test
```

The library includes comprehensive tests for all endpoints:

- Unit tests for each endpoint method
- Integration tests for the complete API workflow
- Error handling tests
- Pagination tests

## Rate Limiting

The AniList API has rate limiting. The client handles basic error responses, but you should implement your own rate limiting logic for production applications.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
