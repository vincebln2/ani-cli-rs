/*  Core data structures for scraper, cli, and player
    - Act as shared interface between fetcher,
    input parser, rendering output, and playing content
*/
use std::fmt;
use reqwest;

#[derive(Debug)]
pub enum AppError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    JsonRequestError(reqwest::Error),
    ClientError(String),
    ApiError(String),
    ParsingError(String),
    NoStreamsAvailable,
    NoEpisodesAvailable,
    UnknownError(String),
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::RequestError(e) => write!(f, "Request error: {}", e),
            AppError::JsonError(e) => write!(f, "JSON parsing error: {}", e),
            AppError::JsonRequestError(e) => write!(f, "JSON request error: {}", e),
            AppError::ClientError(e) => write!(f, "Client error: {}", e),
            AppError::ApiError(e) => write!(f, "API error: {}", e),
            AppError::ParsingError(e) => write!(f, "Parsing error: {}", e),
            AppError::NoStreamsAvailable => write!(f, "No streams available"),
            AppError::NoEpisodesAvailable => write!(f, "No episodes available"),
            AppError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranslationType {
    Sub,
    Dub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeMeta {
    pub number: f32,
    pub released: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anime {
    pub id: String,
    pub title: String,
    pub available_translations: Vec<TranslationType>,
    pub episode_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpisodeStream {
    pub quality: u16,
    pub url: String,
    pub provider: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectedAnime {
    pub anime: Anime,
    pub translation: TranslationType,
    pub episodes: Vec<EpisodeMeta>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEntry {
    pub anime_id: String,
    pub last_episode: f32,
    pub translation: TranslationType,
}

impl fmt::Display for TranslationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranslationType::Sub => write!(f, "Sub"),
            TranslationType::Dub => write!(f, "Dub"),
        }
    }
}
