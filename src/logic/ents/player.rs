use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::logic::entities;

#[derive(Component, Default, Debug, Hash)]
#[component(storage = "SparseSet")]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    pub label: Player
}

pub struct PlayerRegistry;
impl Plugin for PlayerRegistry {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>(&entities::PLAYER);
    }
}
