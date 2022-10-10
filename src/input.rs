use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::{
    prelude::*,

    plugin::InputManagerSystem
};

use super::logic::ents::Player;

pub struct InputRegistry;
impl Plugin for InputRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<InputAction>::default())
            .add_system_set_to_stage(CoreStage::PreUpdate, ConditionSet::new()
                .before(InputManagerSystem::Update)

                .with_system(bundle_input::<Player>)
                .into()
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    Attack,
    Jump,
    Move
}

pub fn input_map() -> InputMap<InputAction> {
    InputMap::default()
        .insert(MouseButton::Left, InputAction::Attack)
        .insert(VirtualDPad {
            up: KeyCode::W.into(),
            down: KeyCode::S.into(),
            left: KeyCode::A.into(),
            right: KeyCode::D.into()
        }, InputAction::Move)
        .insert(KeyCode::Space, InputAction::Jump)
        .build()
}

pub fn bundle_input<T: Component>(mut query: Query<&mut InputMap<InputAction>, Added<T>>) {
    for mut map in &mut query {
        *map = input_map();
    }
}
