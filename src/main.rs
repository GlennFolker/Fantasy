#![feature(let_chains)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod incl;

pub mod assets;
pub mod core;
pub mod ents;
pub mod ui;

use incl::*;

fn main() {
    App::new()
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
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ImageSettings::default_nearest())

        .add_loopless_state(FState::PreLoading)
        .add_plugins_with(DefaultPlugins, |group| {
            group.add_before::<AssetPlugin, _>(EmbeddedAssetPlugin)
        })
        .add_plugin(AudioPlugin)
        .add_plugin(LdtkPlugin)

        .add_plugin(AssetsRegistry)
        .add_plugin(CoreRegistry)
        .add_plugin(EntsRegistry)
        .add_plugin(UiRegistry)

        .run();
}
