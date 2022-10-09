use bevy::prelude::*;

pub mod menu;

mod func;
pub use func::*;

use menu::MenuUiRegistry;

pub struct UiRegistry;
impl Plugin for UiRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MenuUiRegistry)
            .add_plugin(UiFuncRegistry);
    }
}
