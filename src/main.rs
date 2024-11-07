use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowResolution};

use config::*;

mod components;
mod config;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                title: TITLE.to_string(),
                resizable: false,
                mode: WindowMode::Windowed,
                resize_constraints: bevy::window::WindowResizeConstraints {
                    min_width: WINDOW_WIDTH,
                    max_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                    max_height: WINDOW_HEIGHT,
                },
                ..Default::default()
            }),
            ..Default::default()
        }))
        .init_resource()
        //.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .run();
}
