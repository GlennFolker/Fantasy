use crate::incl::*;

pub struct UiInteractRegistry;
impl Plugin for UiInteractRegistry {
    fn build(&self, app: &mut App) {
        app
            .add_system(sys_interact_change)

            .add_system(sys_interact_col_target)
            .add_system(sys_interact_col_lerp.after(sys_interact_col_target));
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct UiInteract {
    pub current: Interaction,
    pub last: Interaction
}

impl UiInteract {
    pub fn clicked(&self) -> bool {
        self.last == Interaction::Clicked && self.current == Interaction::Hovered
    }
}

pub fn sys_interact_change(mut query: Query<(&Interaction, &mut UiInteract), Changed<Interaction>>) {
    for (interaction, mut track) in &mut query {
        track.last = track.current;
        track.current = *interaction;
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct UiInteractCol {
    pub normal: Color,
    pub hovered: Option<Color>,
    pub clicked: Option<Color>
}

#[derive(Component, Clone, Debug)]
pub struct UiInteractColSpeed {
    pub hover_speed: f32,
    pub click_speed: f32
}

impl Default for UiInteractColSpeed {
    fn default() -> Self {
        Self {
            hover_speed: 10.0,
            click_speed: 16.0
        }
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct UiInteractColProg {
    pub hovered: bool,
    pub hover_prog: f32,

    pub clicked: bool,
    pub click_prog: f32
}

#[derive(Bundle, Default, Clone)]
pub struct UiInteractColBundle {
    pub col: UiInteractCol,
    pub speed: UiInteractColSpeed,
    pub prog: UiInteractColProg
}

pub fn sys_interact_col_target(mut interacted: Query<(&UiInteract, &mut UiInteractColProg), Changed<UiInteract>>) {
    for (interaction, mut prog) in &mut interacted {
        match interaction.current {
            Interaction::None => {
                prog.hovered = false;
                prog.clicked = false;
            },

            Interaction::Hovered => {
                prog.hovered = true;
                prog.clicked = false;
            },

            Interaction::Clicked => prog.clicked = true
        }

        if interaction.last == Interaction::None && interaction.current == Interaction::None {
            prog.hover_prog = 0.0;
            prog.click_prog = 0.0;
        } else if interaction.last == Interaction::Clicked && interaction.current == Interaction::None {
            prog.hover_prog = 0.0;
        }
    }
}

pub fn sys_interact_col_lerp(
    mut to_lerp: Query<(&UiInteractCol, &UiInteractColSpeed, &mut UiColor, &mut UiInteractColProg)>,
    time: Res<Time>
) {
    for (def, speed, mut col, mut prog) in &mut to_lerp {
        let normal = def.normal.as_rgba_linear();
        let hovered = def.hovered.unwrap_or(normal).as_rgba_linear();
        let clicked = def.clicked.unwrap_or(hovered).as_rgba_linear();

        let d = time.delta_seconds();
        prog.hover_prog = (prog.hover_prog + speed.hover_speed * d * if prog.hovered { 1.0 } else { -1.0 })
            .max(0.0)
            .min(1.0);

        prog.click_prog = (prog.click_prog + speed.click_speed * d * if prog.clicked { 1.0 } else { -1.0 })
            .max(0.0)
            .min(1.0);

        let mut r = normal.r();
        r = r + (hovered.r() - r) * prog.hover_prog;
        r = r + (clicked.r() - r) * prog.click_prog;

        let mut g = normal.g();
        g = g + (hovered.g() - g) * prog.hover_prog;
        g = g + (clicked.g() - g) * prog.click_prog;

        let mut b = normal.b();
        b = b + (hovered.b() - b) * prog.hover_prog;
        b = b + (clicked.b() - b) * prog.click_prog;

        let mut a = normal.a();
        a = a + (hovered.a() - a) * prog.hover_prog;
        a = a + (clicked.a() - a) * prog.click_prog;

        *col = Color::rgba_linear(r, g, b, a).as_rgba().into();
    }
}
