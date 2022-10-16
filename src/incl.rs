pub use bevy::{
    prelude::*,

    asset::AssetPlugin,
    app::AppExit,
    render::texture::ImageSettings,
    window::{
        MonitorSelection,
        WindowId,
        WindowMode, PresentMode,
        WindowResizeConstraints
    },
    winit::WinitWindows
};

pub use bevy_asset_loader::prelude::*;
pub use bevy_ecs_ldtk::prelude::*;
pub use bevy_embedded_assets::EmbeddedAssetPlugin;
pub use bevy_kira_audio::prelude::*;
pub use iyes_loopless::prelude::*;
pub use iyes_progress::ProgressPlugin;

pub use leafwing_input_manager::{
    prelude::*,

    plugin::InputManagerSystem
};

pub use winit::window::Icon;

pub use crate::{
    assets::{
        AssetsRegistry, FAssetRegistry,
        GameAtlas, GameFonts, GameWorld
    },
    core::{
        CoreRegistry, ControlRegistry, InputRegistry,
        FState,
        InputAction,

        entities, levels
    },
    ents::{
        EntsRegistry, PlayerRegistry,

        PlayerBundle, Player
    },
    ui::{
        UiRegistry,

        comp::{
            UiCompRegistry,
            UiInteractRegistry,

            UiInteract,

            UiInteractColBundle,
            UiInteractCol, UiInteractColSpeed, UiInteractColProg
        },

        frag::{
            UiFragRegistry,
            MenuUiRegistry
        }
    }
};
