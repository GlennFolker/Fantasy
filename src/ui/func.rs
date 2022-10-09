use bevy::prelude::*;

#[derive(Component, Default, Clone, Debug)]
pub struct InteractColor {
    pub normal: Color,
    pub clicked: Option<Color>,
    pub hovered: Option<Color>
}

pub fn interact_color(mut interacted: Query<(&Interaction, &InteractColor, &mut UiColor), Changed<Interaction>>) {
    for (interaction, def, mut color) in &mut interacted {
        *color = match *interaction {
            Interaction::Clicked => def.clicked.unwrap_or(def.hovered.unwrap_or(def.normal)),
            Interaction::Hovered => def.hovered.unwrap_or(def.normal),
            Interaction::None => def.normal
        }.into();
    }
}

pub struct UiFuncRegistry;
impl Plugin for UiFuncRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_system(interact_color);
    }
}
