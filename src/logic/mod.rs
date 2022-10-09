pub mod ents;

mod control;
mod ldtk;
pub use control::*;
pub use ldtk::*;

use bevy::prelude::*;
use ents::EntsRegistry;

pub struct LogicRegistry;
impl Plugin for LogicRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ControlRegistry)
            .add_plugin(EntsRegistry);
    }
}
