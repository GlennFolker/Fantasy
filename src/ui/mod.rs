pub mod comp;
pub mod frag;

use crate::incl::*;

pub struct UiRegistry;
impl Plugin for UiRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(UiCompRegistry)
            .add_plugin(UiFragRegistry);
    }
}
