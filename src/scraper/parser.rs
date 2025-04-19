/*
    Parses useful information from jsons returned from api
*/
use crate::models::{Anime, EpisodeMeta, EpisodeStream, TranslationType};
use serde_json::Value;

// parses returned search results from api and constructs a list of anime names
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

// Pick episodes from sub or dub depending on user input, return list of released episodes
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

// Extracts usable EpisodeStream entries from gql response
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
