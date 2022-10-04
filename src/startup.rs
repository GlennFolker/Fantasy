use bevy::{
    prelude::*,

    asset::AssetPlugin,
    render::texture::ImageSettings,
    window::{
        MonitorSelection,
        WindowMode, PresentMode
    }
};

use bevy_ecs_ldtk::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::*;

use super::assets::*;

pub struct StartupPlugin;
impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(WindowDescriptor {
                title: "Fantasy".to_string(),

                width: 800f32,
                height: 600f32,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,

                ..default()
            })
            .insert_resource(Msaa {
                samples: 1
            })
            .insert_resource(ImageSettings::default_nearest())
            .insert_resource(LdtkSettings {
                set_clear_color: SetClearColor::FromLevelBackground,
                ..default()
            })
            .insert_resource(LevelSelection::Index(0))

            .add_plugins_with(DefaultPlugins, |group| {
                group.add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
            })
            .add_plugin(GameAssetsPlugin)
            .add_plugin(LdtkPlugin)
            .add_plugin(AudioPlugin);
    }
}
