use crate::incl::*;

pub struct ControlRegistry;
impl Plugin for ControlRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_exit_system(FState::PreLoading, sys_setup)
            .add_enter_system(FState::InGame, sys_init_world)
            
            .add_system(sys_pan_camera.run_in_state(FState::InGame));
    }
}

pub fn sys_setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scale = 1.0 / 3.0;

    commands.spawn_bundle(cam);
}

pub fn sys_init_world(mut commands: Commands, world: Res<GameWorld>) {
    commands.insert_resource(LevelSelection::Iid(levels::LEVEL.to_string()));
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: world.clone_weak(),
        ..Default::default()
    });
}

pub fn sys_pan_camera(mut camera: Query<&mut Transform, With<Camera2d>>, player: Query<&GlobalTransform, With<Player>>) {
    if let Ok(mut camera) = camera.get_single_mut() && let Ok(player) = player.get_single() {
        let (scl, rot, trns) = player.to_scale_rotation_translation();
        camera.translation = trns;
        camera.rotation = rot;
        camera.scale = scl;
    }
}
