use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    GameWorld,
    GameState
};

use super::levels;

pub struct ControlRegistry;
impl Plugin for ControlRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::InGame, init_world);
    }
}

pub fn init_world(mut commands: Commands, world: Res<GameWorld>) {
    commands.insert_resource(LevelSelection::Iid(levels::LEVEL.to_string()));
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: world.clone_weak(),
        ..Default::default()
    });
}
