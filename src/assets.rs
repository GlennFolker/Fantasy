use bevy::{
    prelude::*,

    window::WindowId,
    winit::WinitWindows
};

use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::{ProgressPlugin};
use winit::window::Icon;

use super::state::*;

pub struct GameAtlas(Handle<TextureAtlas>);
pub struct GameWorld(Handle<LdtkAsset>);

pub struct GameAssetsPlugin;
impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(GameState::PreLoading)
            .add_loading_state(LoadingState::new(GameState::PreLoading)
                .continue_to_state(GameState::Loading)
                .with_collection::<GamePreAssets>()
            )
            .add_loading_state(LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Running(RunState::Menu))
                .with_collection::<GameAssets>()
            )

            .add_plugin(ProgressPlugin::new(GameState::PreLoading))
            .add_plugin(ProgressPlugin::new(GameState::Loading))

            .add_exit_system(GameState::PreLoading, pre_loaded)
            .add_exit_system(GameState::Loading, loaded);
    }
}

fn pre_loaded(pre_assets: Res<GamePreAssets>, images: Res<Assets<Image>>, windows: NonSendMut<WinitWindows>) {
    let icon = images.get(&pre_assets.icon).expect("Icon not loaded.");
    windows
        .get_window(WindowId::primary()).expect("Failed to get primary window.")
        .set_window_icon(Some(Icon::from_rgba(
            icon.data.clone(),
            icon.texture_descriptor.size.width,
            icon.texture_descriptor.size.height
        ).expect("Couldn't create window icon.")));
}

fn loaded(
    mut commands: Commands,
    pre_assets: Res<GamePreAssets>, assets: Res<GameAssets>,
    mut images: ResMut<Assets<Image>>, mut atlases: ResMut<Assets<TextureAtlas>>
) {
    commands.insert_resource(GameAtlas({
        let mut builder = TextureAtlasBuilder::default();
        builder.add_texture(pre_assets.icon.clone_weak(), images.get(&pre_assets.icon).expect("Icon not loaded."));
        for handle in &assets.all_sprites {
            builder.add_texture(handle.clone_weak(), images.get(&handle).expect("Sprite handle invalid."));
        }

        atlases.add(builder.finish(&mut images).expect("Couldn't generate texture atlas."))
    }));

    commands.insert_resource(GameWorld(assets.world.clone()));

    commands.remove_resource::<GamePreAssets>();
    commands.remove_resource::<GameAssets>();
}

#[derive(AssetCollection)]
struct GamePreAssets {
    #[asset(path = "icon.png")]
    icon: Handle<Image>
}

#[derive(AssetCollection)]
struct GameAssets {
    #[asset(path = "sprites", collection(typed))]
    all_sprites: Vec<Handle<Image>>,
    #[asset(path = "world.ldtk")]
    world: Handle<LdtkAsset>
}
