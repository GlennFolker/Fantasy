#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

mod assets;
mod startup;
mod state;

pub use assets::*;
pub use startup::*;
pub use state::*;

fn main() {
    App::new()
        .add_plugin(StartupPlugin)
        .run();
}
