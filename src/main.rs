#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod logic;
pub mod ui;

mod assets;
mod game;
pub use assets::*;
pub use game::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .run();
}
