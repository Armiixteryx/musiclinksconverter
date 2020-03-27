pub mod deezer;
pub mod spotify;

use url::Url;

#[derive(Debug)]
pub enum Services {
    Deezer,
    Spotify
}

trait Constants<'a> {
    const URL_PLAYER_HOST: &'static str;
    fn music_object_type() -> Vec<&'a str>;
    const ID_LEN: usize;
}

pub struct UrlService {
    pub service: Services,
    pub id: String
}

pub struct UrlData {
    pub artist: String,
    pub track: String,
}

pub fn get_service<'a>(user_url: &str) -> Result<UrlService, &'static str> {
    const ERROR_MSG: &'static str = "This is not a valid URL.";
    
    let service;
    let mut id = String::new();

    //let player_host;
    let object_type;
    let service_id_len;

    let mut url = Url::parse(user_url).unwrap();

    let scheme = url.scheme();

    // Checking scheme
    if scheme != "https" {
        if scheme != "http" {
            return Err("I only accept http/https schemes.")
        } else {
            if let Err(()) = url.set_scheme("https") {
                return Err("Internal error.")
            }
        }
    }

    // Checking service by host
    if let Some(host) = url.host_str() {
        match host {
            deezer::Metadata::URL_PLAYER_HOST => {
                service = Services::Deezer;
                object_type = deezer::Metadata::music_object_type();
                service_id_len = deezer::Metadata::ID_LEN;
            },
            spotify::Metadata::URL_PLAYER_HOST => {
                service = Services::Spotify;
                object_type = spotify::Metadata::music_object_type();
                service_id_len = spotify::Metadata::ID_LEN;
            },
            _ => return Err("The url does not contain a supported service")
        }
    } else {
        return Err(ERROR_MSG)
    }
    
    // Check url object
    if let Some(mut path) = url.path_segments() {
        if let Some(query) = path.next() {
            
            if !object_type.iter().any(|&x| x == query) {
                return Err("This is not a song, an album nor an artist url");
            }
        }
        
        // Check id
        if let Some(url_id) = path.next() {
            if url_id.len() != service_id_len {
                return Err(ERROR_MSG);
            } else {
                id = url_id.to_string();
            }
        } else {
            return Err(ERROR_MSG);
        }
    }
    
    Ok(UrlService {
        service,
        id
    })
}


/*
pub trait MusicService {
    fn generate_url(data: &UrlData) -> String;
    fn analyze_url(url: &str) -> UrlData;
}
*/