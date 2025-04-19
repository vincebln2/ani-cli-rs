const BASE_API: &str = "https://api.allanime.day/api";
const REFERER: &str = "https://allmanga.to";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0";

use crate::models::TranslationType;
use reqwest::{Client, header};
use serde_json::{Value, json};

pub fn build_http_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, USER_AGENT.parse().unwrap());
    headers.insert(header::REFERER, REFERER.parse().unwrap());

    Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build client")
}

// Sends the query for anime names, api returns anime with available episodes
pub async fn search_anime(client: &Client, query: &str) -> Result<Value, reqwest::Error> {
    let gql_query = r#"
        query(
            $search: SearchInput,
            $limit: Int,
            $page: Int,
            $translationType: VaildTranslationTypeEnumType,
            $countryOrigin: VaildCountryOriginEnumType
        ) {
            shows(
                search: $search,
                limit: $limit,
                page: $page,
                translationType: $translationType,
                countryOrigin: $countryOrigin
            ) {
                edges {
                    _id
                    name
                    availableEpisodes
                    __typename
                }
            }
        }
    "#;

    let body = json!({
        "query": gql_query,
        "variables": {
            "search": {
                "allowAdult": false,
                "allowUnknown": false,
                "query": query,
            },
            "limit": 40,
            "page": 1,
            "translationType": "sub",
            "countryOrigin": "ALL"
        }
    });

    let resp = client.post(BASE_API).json(&body).send().await?;

    let data: Value = resp.json().await?;
    Ok(data)
}

// Sends a query to get the episodes for the anime
pub async fn get_episode_list(client: &Client, anime_id: &str) -> Result<Value, reqwest::Error> {
    let gql_query = r#"
        query ($showId: String!) {
            show(_id: $showId) {
                _id
                availableEpisodesDetail
            }
        }
    "#;

    let body = json!({
        "query": gql_query,
        "variables": {
            "showId": anime_id
        }
    });

    let response = client
        .post("https://api.allanime.day/api")
        .json(&body)
        .send()
        .await?;

    let data: Value = response.json().await?;
    Ok(data)
}

// returns stream link for given anime, episode number, and translation type (sub or dub)
pub async fn fetch_episode_sources(
    client: &Client,
    anime_id: &str,
    episode_number: &str,
    translation: TranslationType,
) -> Result<Value, reqwest::Error> {
    let gql_query = r#"
            query (
                $showId: String!,
                $translationType: VaildTranslationTypeEnumType!,
                $episodeString: String!
            ) {
                episode(
                    showId: $showId,
                    translationType: $translationType,
                    episodeString: $episodeString
                ) {
                    episodeString
                    sourceUrls
                }
            }
        "#;

    let translation_str = match translation {
        TranslationType::Sub => "sub",
        TranslationType::Dub => "dub",
    };

    let body = json!({
        "query": gql_query,
        "variables": {
            "showId": anime_id,
            "translationType": translation_str,
            "episodeString": episode_number
        }
    });

    let response = client
        .post("https://api.allanime.day/api")
        .json(&body)
        .send()
        .await?;

    let data: Value = response.json().await?;
    Ok(data)
}
