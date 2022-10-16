mod control;
mod ldtk;
mod input;
mod state;
pub use control::*;
pub use ldtk::*;
pub use input::*;
pub use state::*;

use crate::incl::*;

pub struct CoreRegistry;
impl Plugin for CoreRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(ControlRegistry)
            .add_plugin(InputRegistry);
    }
}
