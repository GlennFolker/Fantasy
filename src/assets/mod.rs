mod asset;
pub use asset::*;

use crate::incl::*;

pub struct AssetsRegistry;
impl Plugin for AssetsRegistry {
    fn build(&self, app: &mut App) {
        app.add_plugin(FAssetRegistry);
    }
}
