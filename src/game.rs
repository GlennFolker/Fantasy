use bevy::{
    prelude::*,

    asset::AssetPlugin,
    render::texture::ImageSettings,
    window::{
        MonitorSelection,
        WindowMode, PresentMode,
        WindowResizeConstraints
    }
};

use bevy_ecs_ldtk::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::*;
use iyes_loopless::prelude::*;

use super::{
    AssetsRegistry,
    GameState,

    logic::LogicRegistry,
    ui::UiRegistry
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "Fantasy".to_string(),

                width: 800.0,
                height: 600.0,
                resize_constraints: WindowResizeConstraints {
                    min_width: 800.0,
                    min_height: 600.0,
                    ..default()
                },
                position: WindowPosition::Centered(MonitorSelection::Primary),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoNoVsync,

                ..default()
            })
            .insert_resource(ClearColor(Color::BLACK))
            .insert_resource(Msaa {
                samples: 1
            })
            .insert_resource(ImageSettings::default_nearest())

            .add_plugins_with(DefaultPlugins, |group| {
                group.add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
            })
            .add_plugin(LdtkPlugin)
            .add_plugin(AudioPlugin)

            .add_plugin(AssetsRegistry)
            .add_plugin(LogicRegistry)
            .add_plugin(UiRegistry)

            .add_exit_system(GameState::PreLoading, setup);
    }
}

fn setup(mut commands: Commands) {
    let mut cam = Camera2dBundle::default();
    cam.projection.scale = 1.0 / 3.0;

    commands.spawn_bundle(cam);
}
