use crate::incl::*;

pub struct FAssetRegistry;
impl Plugin for FAssetRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(LoadingState::new(FState::PreLoading)
                .continue_to_state(FState::Loading)
                .with_collection::<FPreAssets>()
                .init_resource::<Preloaded>()
            )
            .add_loading_state(LoadingState::new(FState::Loading)
                .continue_to_state(FState::Menu)
                .with_collection::<FAssets>()
                .init_resource::<GameAtlas>()
                .init_resource::<GameWorld>()
                .init_resource::<GameFonts>()
            )

            .add_plugin(ProgressPlugin::new(FState::PreLoading))
            .add_plugin(ProgressPlugin::new(FState::Loading))

            .add_exit_system(FState::Loading, sys_cleanup);
    }
}

#[derive(Deref, DerefMut)]
pub struct GameAtlas(pub Handle<TextureAtlas>);
#[derive(Deref, DerefMut)]
pub struct GameWorld(pub Handle<LdtkAsset>);
pub struct GameFonts {
    pub script: Handle<Font>
}

impl FromWorld for GameAtlas {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let assets = world.get_resource::<FAssets>().unwrap();
        let mut images = world.get_resource_mut::<Assets<Image>>().unwrap();
        let mut atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        let mut builder = TextureAtlasBuilder::default();
        builder.add_texture(assets.icon.clone_weak(), images.get(&assets.icon).unwrap());
        for handle in &assets.all_sprites {
            builder.add_texture(handle.clone_weak(), images.get(&handle).unwrap());
        }

        Self(atlases.add(builder.finish(&mut images).unwrap()))
    }
}

impl FromWorld for GameWorld {
    fn from_world(world: &mut World) -> Self {
        Self(world.get_resource::<FAssets>().unwrap().world.clone())
    }
}

impl FromWorld for GameFonts {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<FAssets>().unwrap();
        Self {
            script: assets.font_script.clone()
        }
    }
}

#[derive(AssetCollection)]
struct FPreAssets {
    #[asset(path = "icon.png")]
    icon: Handle<Image>
}

#[derive(AssetCollection)]
struct FAssets {
    #[asset(path = "icon.png")]
    icon: Handle<Image>,
    #[asset(path = "sprites", collection(typed))]
    all_sprites: Vec<Handle<Image>>,

    #[asset(path = "world.ldtk")]
    world: Handle<LdtkAsset>,

    #[asset(path = "fonts/press_start.ttf")]
    font_script: Handle<Font>
}

struct Preloaded;
impl FromWorld for Preloaded {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let pre_assets = world.get_resource::<FPreAssets>().unwrap();
        let images = world.get_resource::<Assets<Image>>().unwrap();
        let windows = world.get_non_send_resource_mut::<WinitWindows>().unwrap();

        let icon = images.get(&pre_assets.icon).unwrap();
        windows
            .get_window(WindowId::primary()).expect("Failed to get primary window.")
            .set_window_icon(Some(Icon::from_rgba(
                icon.data.clone(),
                icon.texture_descriptor.size.width,
                icon.texture_descriptor.size.height
            ).expect("Couldn't create window icon.")));

        Self
    }
}

pub fn sys_cleanup(mut commands: Commands) {
    commands.remove_resource::<FPreAssets>();
    commands.remove_resource::<FAssets>();
    commands.remove_resource::<Preloaded>();
}
