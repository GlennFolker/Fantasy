use bevy::{
    prelude::*,

    app::AppExit
};

use iyes_loopless::prelude::*;

use crate::{
    GameFonts,
    GameState, RunState
};

use super::InteractColor;

#[derive(Component, Debug, Hash)]
#[component(storage = "SparseSet")]
pub struct MenuUi;

#[derive(Component, Debug, Hash)]
pub enum MenuButton {
    Play,
    Options,
    Exit
}

const BUTTON_COLOR: InteractColor = InteractColor {
    normal: Color::rgb(0.15, 0.15, 0.15),
    clicked: Some(Color::rgb(0.25, 0.25, 0.25)),
    hovered: Some(Color::rgb(0.35, 0.75, 0.35))
};

pub fn init(mut commands: Commands, fonts: Res<GameFonts>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .insert(MenuUi)
        .with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                    "Fantasy",
                    TextStyle {
                        font: fonts.script.clone(),
                        font_size: 64.0,
                        color: Color::WHITE
                    }
                )
                .with_style(Style {
                    size: Size::new(Val::Auto, Val::Px(64.0)),
                    margin: UiRect::all(Val::Px(48.0)),
                    ..default()
                })
            );

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    flex_grow: 1.0,
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            });

            let button_style = Style {
                size: Size::new(Val::Px(320.0), Val::Undefined),
                margin: UiRect::new(Val::Px(32.0), Val::Px(10.0), Val::Px(10.0), Val::Px(10.0)),
                align_items: AlignItems::Center,
                ..default()
            };

            let button_text_style = TextStyle {
                font: fonts.script.clone(),
                font_size: 32.0,
                color: Color::WHITE
            };

            let text_style = Style {
                margin: UiRect::all(Val::Px(4.0)),
                ..default()
            };

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(BUTTON_COLOR)
                .insert(MenuButton::Play)
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Play",
                            button_text_style.clone()
                        )
                        .with_style(text_style.clone())
                    );
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(BUTTON_COLOR)
                .insert(MenuButton::Options)
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Options",
                            button_text_style.clone()
                        )
                        .with_style(text_style.clone())
                    );
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert(BUTTON_COLOR)
                .insert(MenuButton::Exit)
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Exit",
                            button_text_style.clone()
                        )
                        .with_style(text_style.clone())
                    );
                });

            parent.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Undefined, Val::Px(22.0)),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            });
        });
}

pub fn dispose(mut commands: Commands, menu_uis: Query<Entity, With<MenuUi>>) {
    for entity in &menu_uis {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update_buttons(
    mut commands: Commands,
    buttons: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, button) in &buttons {
        if *interaction == Interaction::Clicked {
            match *button {
                MenuButton::Play => commands.insert_resource(NextState(GameState::Running(RunState::InGame(true)))),
                MenuButton::Options => {},
                MenuButton::Exit => exit.send(AppExit)
            }
        }
    }
}

pub struct MenuUiRegistry;
impl Plugin for MenuUiRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(GameState::Running(RunState::Menu), init)
            .add_exit_system(GameState::Running(RunState::Menu), dispose)
            .add_system(update_buttons.run_in_state(GameState::Running(RunState::Menu)));
    }
}
