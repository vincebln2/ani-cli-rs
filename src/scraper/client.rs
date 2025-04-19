
use crate::models::{TranslationType, Result, AppError};
use reqwest::{Client, header};
use serde_json::{Value, json};
use std::time::Duration;

const BASE_API: &str = "https://api.allanime.day/api";
const REFERER: &str = "https://allmanga.to";
const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0";

// API client for interacting with the anime service
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    // Creates a new API client with proper headers and timeout settings
    pub fn new() -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        let user_agent = header::HeaderValue::from_str(USER_AGENT)
            .map_err(|e| AppError::ClientError(format!("Invalid user agent: {}", e)))?;

        let referer = header::HeaderValue::from_str(REFERER)
            .map_err(|e| AppError::ClientError(format!("Invalid referer: {}", e)))?;

        headers.insert(header::USER_AGENT, user_agent);
        headers.insert(header::REFERER, referer);

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| AppError::ClientError(format!("Failed to build client: {}", e)))?;

        Ok(Self { client })
    }

    // Searches for anime matching the given query string
    pub async fn search_anime(&self, query: &str) -> Result<Value> {
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

        self.send_request(body).await
    }

    /// Retrieves the episode list for a specific anime
    pub async fn get_episode_list(&self, anime_id: &str) -> Result<Value> {
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

        self.send_request(body).await
    }

    // Fetches streaming sources for a specific episode of an anime
    pub async fn fetch_episode_sources(
        &self,
        anime_id: &str,
        episode_number: &str,
        translation: TranslationType,
    ) -> Result<Value> {
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

        self.send_request(body).await
    }

    // Sends a GraphQL request to the API with basic retry logic
    async fn send_request(&self, body: serde_json::Value) -> Result<Value> {
        const MAX_RETRIES: u8 = 2;
        let mut retry_count = 0;
        let mut last_error = None;

        while retry_count <= MAX_RETRIES {
            match self.try_send_request(&body).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    retry_count += 1;
                    last_error = Some(e);

                    if retry_count <= MAX_RETRIES {
                        // Simple delay between retries
                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| AppError::UnknownError("Unknown error during API request".to_string())))
    }

    // Attempts to send a single request to the API
    async fn try_send_request(&self, body: &serde_json::Value) -> Result<Value> {
        let response = self.client
            .post(BASE_API)
            .json(body)
            .send()
            .await
            .map_err(|e| AppError::RequestError(e))?;

        if !response.status().is_success() {
            return Err(AppError::ApiError(format!(
                "API returned error status: {}",
                response.status()
            )));
        }

        response.json::<Value>()
            .await
            .map_err(|e| AppError::JsonRequestError(e))
    }
}
