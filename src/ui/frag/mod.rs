mod menu;
pub use menu::*;

use crate::incl::*;

pub struct UiFragRegistry;
impl Plugin for UiFragRegistry {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuUiRegistry);
    }
}
