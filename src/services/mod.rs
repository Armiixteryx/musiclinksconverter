pub mod deezer;
pub mod spotify;

#[derive(Debug)]
pub enum Services {
    Deezer,
    Spotify,
    Unsupported
}

pub enum MusicType {
    Track,
    Album,
    Artist
}

pub struct UrlData {
    pub artist: String,
    pub track: String,
}

/*
pub trait MusicService {
    fn generate_url(data: &UrlData) -> String;
    fn analyze_url(url: &str) -> UrlData;
}
*/