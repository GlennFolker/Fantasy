use crate::incl::*;

pub struct MenuUiRegistry;
impl Plugin for MenuUiRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_enter_system(FState::Menu, sys_init)
            .add_exit_system(FState::Menu, sys_dispose)
            .add_system(sys_update_buttons.run_in_state(FState::Menu));
    }
}

#[derive(Component, Debug, Hash)]
#[component(storage = "SparseSet")]
pub struct MenuUi;

#[derive(Component, Debug, Hash)]
pub enum MenuButton {
    Play,
    Options,
    Exit
}

pub fn sys_init(mut commands: Commands, fonts: Res<GameFonts>) {
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

            let button_color = UiInteractColBundle {
                col: UiInteractCol {
                    normal: Color::rgb(0.15, 0.15, 0.15),
                    clicked: Some(Color::rgb(0.25, 0.25, 0.25)),
                    hovered: Some(Color::rgb(0.35, 0.75, 0.35))
                },
                ..default()
            };

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    ..default()
                })
                .insert_bundle(button_color.clone())
                .insert(MenuButton::Play)
                .insert(UiInteract::default())
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
                .insert_bundle(button_color.clone())
                .insert(MenuButton::Options)
                .insert(UiInteract::default())
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
                .insert_bundle(button_color.clone())
                .insert(MenuButton::Exit)
                .insert(UiInteract::default())
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

pub fn sys_dispose(mut commands: Commands, menu_uis: Query<Entity, With<MenuUi>>) {
    for entity in &menu_uis {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn sys_update_buttons(
    mut commands: Commands,
    buttons: Query<(&UiInteract, &MenuButton), Changed<UiInteract>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, button) in &buttons {
        if interaction.clicked() {
            match *button {
                MenuButton::Play => commands.insert_resource(NextState(FState::InGame)),
                MenuButton::Options => {},
                MenuButton::Exit => exit.send(AppExit)
            }
        }
    }
}
