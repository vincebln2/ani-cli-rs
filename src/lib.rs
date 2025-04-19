pub mod config;
pub mod models;
pub mod scraper;

pub use models::{
    Anime, EpisodeMeta, EpisodeStream, TranslationType, SelectedAnime, HistoryEntry, AppError, Result
};
