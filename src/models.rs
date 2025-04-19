/*  Core data structures for scraper, cli, and player
    - Act as shared interface between fetcher,
    input parser, rendering output, and playing content

*/
use std::fmt;

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
