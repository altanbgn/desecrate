use bevy::prelude::*;
use std::fs::File;
use std::io::prelude;

use crate::config::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Setting {
    enable_sound: bool,
    enable_music: bool,
}

impl Setting {
    pub fn new(enable_sound: bool, enable_music: bool) -> Self {
        Setting {
            enable_sound,
            enable_music
        }
    }

    pub fn get_enable_sound(&self) -> bool {
        self.enable_sound
    }

    pub fn get_enable_music(&self) -> bool {
        self.enable_music
    }

    pub fn set_enable_sound(&mut self, enable_sound: bool) {
        self.enable_sound = enable_sound;
    }

    pub fn set_enable_music(&mut self, enable_music: bool) {
        self.enable_music = enable_music;
    }
}
