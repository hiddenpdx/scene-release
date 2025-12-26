use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ParsedRelease {
    pub release: String,
    pub title: String,
    pub title_extra: String,
    pub episode_title: String,
    pub group: String,
    pub year: Option<u16>,
    pub date: Option<String>,
    pub season: Option<u128>,
    pub episode: Option<u128>,
    pub episodes: Vec<u128>,
    pub disc: Option<u8>,
    pub flags: Vec<String>,
    pub source: String,
    pub format: String,
    pub resolution: String,
    pub audio: String,
    pub device: String,
    pub os: String,
    pub version: String,
    pub language: HashMap<String, String>,
    pub tmdb_id: Option<String>,
    pub tvdb_id: Option<String>,
    pub imdb_id: Option<String>,
    pub edition: Option<String>,
    pub hdr: String,
    pub streaming_provider: String,
    #[serde(rename = "type")]
    pub release_type: String,
}


impl ParsedRelease {
    pub fn get(&self, field: &str) -> Option<String> {
        match field {
            "release" => Some(self.release.clone()),
            "title" => Some(self.title.clone()),
            "title_extra" => Some(self.title_extra.clone()),
            "episode_title" => Some(self.episode_title.clone()),
            "group" => Some(self.group.clone()),
            "year" => self.year.map(|y| y.to_string()),
            "date" => self.date.clone(),
            "season" => self.season.map(|s| s.to_string()),
            "episode" => self.episode.map(|e| e.to_string()),
            "episodes" => if self.episodes.is_empty() {
                None
            } else {
                Some(self.episodes.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(","))
            },
            "disc" => self.disc.map(|d| d.to_string()),
            "source" => Some(self.source.clone()),
            "format" => Some(self.format.clone()),
            "resolution" => Some(self.resolution.clone()),
            "audio" => Some(self.audio.clone()),
            "device" => Some(self.device.clone()),
            "os" => Some(self.os.clone()),
            "version" => Some(self.version.clone()),
            "tmdb_id" => self.tmdb_id.clone(),
            "tvdb_id" => self.tvdb_id.clone(),
            "imdb_id" => self.imdb_id.clone(),
            "edition" => self.edition.clone(),
            "hdr" => Some(self.hdr.clone()),
            "streaming_provider" => Some(self.streaming_provider.clone()),
            "type" => Some(self.release_type.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PathInfo {
    pub directory: Option<ParsedRelease>,
    pub season: Option<u128>,
    pub file: ParsedRelease,
    pub full_path: String,
}

