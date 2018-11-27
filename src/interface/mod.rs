/*  
    @Author Julien LE THENO
    @mod Tui : handles the Text user interface
*/
extern crate spotify_api;

pub mod util;
use tui::layout::Rect;

pub struct Albums<'a> {
    pub size: Rect,
    pub albums: Vec<spotify_api::Album<'a>>,
    pub selected: usize,
    pub active: bool,
}

impl<'a> Albums<'a> {
    pub fn new(_albums: Vec<spotify_api::Album>) -> Albums {
        Albums {
            size: Rect::default(),
            albums: _albums,
            selected: 0,
            active: true,
        }
    }
    pub fn add_albums(&mut self, _albums: &mut Vec<spotify_api::Album<'a>>) {
        self.albums.append(_albums);
    }

    pub fn get_selected(&self) -> Option<usize> {
        if self.active {
            Some(self.selected)
        } else {
            None
        }
    }
    pub fn get_selected_album(&self) -> Option<&spotify_api::Album> {
        Some(&self.albums[self.selected])
    }
    pub fn advance(&mut self) {}
}

pub struct Tracks<'a> {
    pub size: Rect,
    pub tracks: Vec<spotify_api::Track<'a>>,
    pub selected: usize,
    pub active: bool,
}
impl<'a> Tracks<'a> {
    pub fn new() -> Tracks<'a> {
        Tracks {
            size: Rect::default(),
            tracks: Vec::new(),
            selected: 0,
            active: false,
        }
    }

    pub fn add_tracks(&mut self, _tracks: & Vec<spotify_api::Track<'a>>) {
        self.tracks.append(&mut _tracks.clone());
    }
    pub fn clear_tracks(&mut self) {
        //self.items.remove(self.items.len());
        self.tracks.clear();
    }

    pub fn get_selected(&self) -> Option<usize> {
        if self.active {
            Some(self.selected)
        } else {
            None
        }
    }
    pub fn get_selected_track(&self) -> Option<&spotify_api::Track> {
        if self.active {
            Some(&self.tracks[self.selected])
        } else {
            None
        }
    }

    pub fn advance(&mut self) {}
}
