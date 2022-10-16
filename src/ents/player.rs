use crate::incl::*;

pub struct PlayerRegistry;
impl Plugin for PlayerRegistry {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>(&entities::PLAYER);
    }
}

#[derive(Component, Default, Debug, Hash)]
#[component(storage = "SparseSet")]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub label: Player,
    #[bundle]
    pub input: InputManagerBundle::<InputAction>
}
