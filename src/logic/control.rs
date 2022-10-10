use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    GameWorld,
    GameState
};

use super::{
    ents::Player,
    levels
};

pub struct ControlRegistry;
impl Plugin for ControlRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::InGame, init_world)
            .add_system(pan_camera.run_in_state(GameState::InGame));
    }
}

pub fn init_world(mut commands: Commands, world: Res<GameWorld>) {
    commands.insert_resource(LevelSelection::Iid(levels::LEVEL.to_string()));
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: world.clone_weak(),
        ..Default::default()
    });
}

pub fn pan_camera(mut camera: Query<&mut Transform, With<Camera2d>>, player: Query<&GlobalTransform, With<Player>>) {
    if let Ok(mut camera) = camera.get_single_mut() && let Ok(player) = player.get_single() {
        let (scl, rot, trns) = player.to_scale_rotation_translation();
        camera.translation = trns;
        camera.rotation = rot;
        camera.scale = scl;
    }
}
