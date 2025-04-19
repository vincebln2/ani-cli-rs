/*
    Parses useful information from JSON returned from API
    with proper error handling
*/
use crate::models::{Anime, EpisodeMeta, EpisodeStream, TranslationType, Result, AppError};
use serde_json::Value;

// Parses returned search results from API and constructs a list of anime names
pub fn parse_search_results(data: &Value) -> Vec<Anime> {
    let mut results = Vec::new();

    if let Some(edges) = data["data"]["shows"]["edges"].as_array() {
        for entry in edges {
            // Loops through anime results from API and pushes a constructed Anime struct to the result
            let id = entry["_id"].as_str().unwrap_or_default().to_string();
            let title = entry["name"].as_str().unwrap_or_default().to_string();

            let mut translations = Vec::new();
            let available = &entry["availableEpisodes"];
            if available["sub"].as_u64().unwrap_or(0) > 0 {
                translations.push(TranslationType::Sub);
            }
            if available["dub"].as_u64().unwrap_or(0) > 0 {
                translations.push(TranslationType::Dub);
            }

            let episode_count = available["sub"]
                .as_u64()
                .or_else(|| available["dub"].as_u64())
                .unwrap_or(0) as usize;

            results.push(Anime {
                id,
                title,
                available_translations: translations,
                episode_count,
            });
        }
    }
    results
}

// Parse episodes with error handling
pub fn parse_episode_list(data: &Value, translation: TranslationType) -> Vec<EpisodeMeta> {
    let mut episodes = Vec::new();

    let key = match translation {
        TranslationType::Sub => "sub",
        TranslationType::Dub => "dub",
    };

    let list = &data["data"]["show"]["availableEpisodesDetail"][key];

    if let Some(array) = list.as_array() {
        for ep in array {
            if let Some(s) = ep.as_str() {
                if let Ok(num) = s.parse::<f32>() {
                    episodes.push(EpisodeMeta {
                        number: num,
                        released: true,
                    });
                }
            }
        }
    }
    episodes
}

// Extracts usable EpisodeStream entries from GQL response
pub fn parse_stream_sources(data: &Value) -> Vec<EpisodeStream> {
    let mut streams = Vec::new();
    let sources = &data["data"]["episode"]["sourceUrls"];

    if let Some(array) = sources.as_array() {
        for entry in array {
            let url = entry["sourceUrl"].as_str().unwrap_or_default();
            let provider = entry["sourceName"].as_str().unwrap_or("Unknown");

            if url.starts_with("http") {
                streams.push(EpisodeStream {
                    url: url.to_string(),
                    provider: provider.to_string(),
                    quality: 0, // Placeholder for now
                });
            }
        }
    }

    streams
}

// For more complex parsing operations that might fail
pub fn parse_complex_data(data: &Value) -> Result<Vec<EpisodeStream>> {
    let sources = &data["data"]["episode"]["sourceUrls"];

    if !sources.is_array() {
        return Err(AppError::ParsingError("Expected sourceUrls to be an array".to_string()));
    }

    let array = sources.as_array().unwrap(); // Safe because we checked above
    if array.is_empty() {
        return Err(AppError::NoStreamsAvailable);
    }

    let mut streams = Vec::new();

    for entry in array {
        let url = entry["sourceUrl"].as_str()
            .ok_or_else(|| AppError::ParsingError("Missing sourceUrl field".to_string()))?;

        let provider = entry["sourceName"].as_str().unwrap_or("Unknown");

        if url.starts_with("http") {
            streams.push(EpisodeStream {
                url: url.to_string(),
                provider: provider.to_string(),
                quality: parse_quality(entry).unwrap_or(0),
            });
        }
    }

    if streams.is_empty() {
        return Err(AppError::NoStreamsAvailable);
    }

    Ok(streams)
}

// Helper function to parse quality information if available
fn parse_quality(entry: &Value) -> Option<u16> {
    entry["quality"].as_str()
        .and_then(|q| q.trim().parse::<u16>().ok())
        .or_else(|| {
            // If there's no explicit quality field, try to infer from other fields
            // This is just a placeholder for demonstration
            Some(720)
        })
}