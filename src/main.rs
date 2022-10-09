#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

pub mod logic;
pub mod ui;

mod assets;
mod game;
mod state;
pub use assets::*;
pub use game::*;
pub use state::*;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .run();
}
