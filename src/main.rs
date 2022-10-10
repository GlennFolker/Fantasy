#![feature(let_chains)]

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod logic;
pub mod ui;

mod assets;
mod game;
mod input;
pub use assets::*;
pub use game::*;
pub use input::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .run();
}
