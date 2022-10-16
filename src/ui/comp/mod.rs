mod interact;
pub use interact::*;

use crate::incl::*;

pub struct UiCompRegistry;
impl Plugin for UiCompRegistry {
    fn build(&self, app: &mut App) {
        app.add_plugin(UiInteractRegistry);
    }
}
