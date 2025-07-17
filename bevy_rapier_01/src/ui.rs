use bevy::prelude::*;
use crate::components::PhysicsStateText;
use crate::resources::PhysicsState;

pub fn ui_setup(mut commands: Commands) {
    commands.spawn(
        TextBundle::from_section(
            "Press R to restart",
            TextStyle {
                font_size: 28.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            top: Val::Px(10.0),
            ..Default::default()
        }),
    );

    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font_size: 18.0,
                color: Color::WHITE,
                ..Default::default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            right: Val::Px(20.0),
            bottom: Val::Px(10.0),
            ..Default::default()
        }),
        PhysicsStateText,
    ));
}

pub fn ui_update_physic_state_text(
    mut q_text: Query<&mut Text, With<PhysicsStateText>>,
    physic_state: Res<PhysicsState>,
) {
    if physic_state.is_changed() {
        let mut text = q_text.single_mut();
        if !physic_state.hash.is_empty() {
            text.sections[0].value = format!(
                "Physic simulation steps: {}. sha256: {}",
                physic_state.step, physic_state.hash
            );
        } else {
            text.sections[0].value = format!("Physic simulation steps: {}", physic_state.step);
        }
    }
}
