/*  
    @Author Julien LE THENO
    @mod EasyAPI : handles the whole Spotify API bindings,
    and the API rights
*/
extern crate failure;
extern crate serde_json;

mod command;
mod files;

use self::command::Command;
use self::serde_json::Value;

pub struct EasyAPI {
    command: Command,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Album {
    pub name: String,
    pub id: String,
}
pub struct Artist {
    pub name: String,
    pub id: String,
}
pub struct Playlist {
    pub name: String,
    pub id: String,
}
impl EasyAPI {
    pub fn new() -> EasyAPI {
        let command = Command::new();
        return EasyAPI { command };
    }
    /// Searches for a playlist and play the first item
    /// found with that name. Works only with playlist now..
    /// TODO extend from playlist mode to any mode..
    pub fn search_and_play_first(
        &mut self,
        type_: &str,
        search: &str,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(search, type_, &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        result = v["playlists"]["items"][0]["id"].to_string(); // just getting the first result here
        result = result[1..].to_string(); // removing last '"'
        result.pop(); // removing first '"'
                      //println!("{}",result);

        self.command.play(result.as_str(), type_, "", "");
        Ok(())
    }

    ///  Get all the current user's albums
    pub fn get_my_albums(&mut self, final_result: &mut Vec<Album>) -> Result<(), failure::Error> {
        for i in 0..5 {
            // SUPER dirty -> TODO get number of album to know how many chunks to get.
            self.get_my_albums_chunk(i * 50, final_result).unwrap();
        }
        Ok(())
    }
    /// Get 50 albums in the user's library with a given offset
    pub fn get_my_albums_chunk(
        &mut self,
        offset: u16,
        final_result: &mut Vec<Album>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_my_albums(offset, &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            let mut album_name = v["items"][x]["album"]["name"].to_string(); // just getting the first result here
            album_name = album_name[1..].to_string(); // removing last '"'
            album_name.pop(); // removing first '"'

            let mut album_id = v["items"][x]["album"]["id"].to_string(); // just getting the first result here
            album_id = album_id[1..].to_string(); // removing last '"'
            album_id.pop(); // removing first '"'

            final_result.push(Album {
                name: album_name,
                id: album_id,
            });
        }
        Ok(())
    }

    ///  Get the track names from a given album id
    pub fn get_tracks_from_album(
        &mut self,
        id_album: &str,
        final_result: &mut Vec<Track>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_tracks_from_album(id_album, &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        // work for playlist, we should verify the JSON out for other types to get the right thing
        let size = v["items"].as_array().unwrap().len();
        for x in 0..size {
            let mut track_name = v["items"][x]["name"].to_string(); // just getting the first result here
            track_name = track_name[1..].to_string(); // removing last '"'
            track_name.pop(); // removing first '"'

            let mut track_id = v["items"][x]["id"].to_string(); // just getting the first result here
            track_id = track_id[1..].to_string(); // removing last '"'
            track_id.pop(); // removing first '"'
            final_result.push(Track {
                name: track_name,
                id: track_id,
            });
        }
        Ok(())
    }
    /// Searches for playlist with the "search" parameter str.
    ///  results are added to the final_result String vector in
    ///  parameter.
    pub fn search_playlist(
        &mut self,
        search: &str,
        final_result: &mut Vec<Playlist>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(search, "playlist", &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["playlists"]["items"].as_array().unwrap().len();
        for x in 0..size {
            let mut playlist_name = v["playlists"]["items"][x]["name"].to_string();
            playlist_name = playlist_name[1..].to_string(); // removing last '"'
            playlist_name.pop(); // removing first '"'

            let mut playlist_id = v["playlists"]["items"][x]["id"].to_string();
            playlist_id = playlist_id[1..].to_string(); // removing last '"'
            playlist_id.pop(); // removing first '"'

            final_result.push(Playlist {
                name: playlist_name,
                id: playlist_id,
            });
        }
        Ok(())
    }
    /// TODO
    /// Not implemented yet
    pub fn search_album(
        &mut self,
        search: &str,
        final_result: &mut Vec<Album>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(search, "album", &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["albums"]["items"].as_array().unwrap().len();
        for x in 0..size {
            let mut album_name = v["albums"]["items"][x]["name"].to_string();
            album_name = album_name[1..].to_string(); // removing last '"'
            album_name.pop(); // removing first '"'

            let mut album_id = v["albums"]["items"][x]["id"].to_string();
            album_id = album_id[1..].to_string(); // removing last '"'
            album_id.pop(); // removing first '"'

            final_result.push(Album {
                name: album_name,
                id: album_id,
            });
        }
        Ok(())
    }
    /// TODO
    /// Not implemented yet
    pub fn search_track(
        &mut self,
        search: &str,
        final_result: &mut Vec<Track>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(search, "track", &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["tracks"]["items"].as_array().unwrap().len();
        for x in 0..size {
            let mut track_name = v["tracks"]["items"][x]["name"].to_string();
            track_name = track_name[1..].to_string(); // removing last '"'
            track_name.pop(); // removing first '"'

            let mut track_id = v["tracks"]["items"][x]["id"].to_string();
            track_id = track_id[1..].to_string(); // removing last '"'
            track_id.pop(); // removing first '"'

            final_result.push(Track {
                name: track_name,
                id: track_id,
            });
        }
        Ok(())
    }
    /// TODO
    /// Not implemented yet
    pub fn search_artist(
        &mut self,
        search: &str,
        final_result: &mut Vec<Artist>,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.search(search, "artist", &mut result);
        let v: Value = serde_json::from_str(result.as_str()).unwrap();
        let size = v["artists"]["items"].as_array().unwrap().len();
        for x in 0..size {
            let mut artist_name = v["artists"]["items"][x]["name"].to_string();
            artist_name = artist_name[1..].to_string(); // removing last '"'
            artist_name.pop(); // removing first '"'

            let mut artist_id = v["artists"]["items"][x]["id"].to_string();
            artist_id = artist_id[1..].to_string(); // removing last '"'
            artist_id.pop(); // removing first '"'

            final_result.push(Artist {
                name: artist_name,
                id: artist_id,
            });
        }
        Ok(())
    }
    /// TODO
    /// Not implemented yet
    pub fn pause(&mut self) -> Result<(), failure::Error> {
        unimplemented!();
    }
    /// TODO
    /// Not implemented yet
    pub fn next(&mut self) -> Result<(), failure::Error> {
        unimplemented!();
    }

    /// Gets the currently playing artist on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_artist(
        &mut self,
        final_result: &mut String,
    ) -> Result<(), failure::Error> {
        let mut result = String::new();
        self.command.get_currently_playing(&mut result);
        if result.len() == 0 {
            *final_result = "".to_string();
        } else {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            result = v["item"]["artists"][0]["name"].to_string();
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            *final_result = result;
        }
        Ok(())
    }

    /// Gets the currently playing track on the final_result argument
    /// final_result setted to "" if no track is playing
    pub fn get_currently_playing_track(
        &mut self,
        final_result: &mut String,
    ) -> Result<(), std::io::Error> {
        let mut result = String::new();
        let errno = self.command.get_currently_playing(&mut result);
        if (errno.is_err()) {
            return errno;
        }

        if result.len() == 0 {
            *final_result = "".to_string();
        } else {
            let v: Value = serde_json::from_str(result.as_str()).unwrap();
            result = v["item"]["name"].to_string();
            result = result[1..].to_string(); // removing last '"'
            result.pop(); // removing first '"'
            *final_result = result;
        }
        Ok(())
    }
    /// Plays a track in a context ( for now just Album..)
    pub fn play_track(
        &mut self,
        track: &Track,
        context: Option<&Album>,
    ) -> Result<(), std::io::Error> {
        match context {
            Some(context) => {
                return self
                    .command
                    .play(track.id.as_str(), "track", context.id.as_str(), "album")
            }
            None => return self.command.play(track.id.as_str(), "track", "", ""),
        }
    }

    /// Refreshes the access token by requesting a new one
    pub fn refresh(&mut self) -> Result<(), failure::Error> {
        let mut refresh_token = String::new();
        let mut base_64_secret = String::new();
        files::load_keys(&mut refresh_token, &mut base_64_secret);
        self.command
            .refresh(base_64_secret.as_str(), refresh_token.as_str());
        Ok(())
    }
}
