use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::game::game_setup;
use crate::resources::*;

const BIRD_START: Vec2 = Vec2::new(-350.0, -120.0);
const SLINGSHOT_ANCHOR: Vec2 = Vec2::new(-350.0, BIRD_START.y);
const SLINGSHOT_MAX_DIST: f32 = 120.0;
const PIXEL_PHYSIC_SCALE: f32 = 10_000.0; // 100 * 100

pub fn game_state_control(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q_boxes: Query<Entity, With<SelectableBox>>,
    q_bird: Query<Entity, With<Bird>>,
    q_camera: Query<Entity, With<Camera2d>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut bird_state: ResMut<BirdState>,
    mut physic_state: ResMut<PhysicsState>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        // Reset game state
        // Despawn all entities in the queries
        for e in q_boxes.iter() {
            commands.entity(e).despawn_recursive();
        }
        for e in q_bird.iter() {
            commands.entity(e).despawn_recursive();
        }
        for e in q_camera.iter() {
            commands.entity(e).despawn_recursive();
        }

        game_setup(commands, meshes, materials);
        bird_state.dragging = false;
        bird_state.launched = false;
        *physic_state = PhysicsState::default();
    }
}

pub fn camera_drag(
    mut q_camera: Query<&mut Transform, With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut last_pos: Local<Option<Vec2>>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    if mouse.pressed(MouseButton::Right) {
        if let Some(cursor) = window.cursor_position() {
            if let Some(last) = *last_pos {
                let delta = cursor - last;
                let mut cam = q_camera.single_mut();
                cam.translation.x -= delta.x;
                cam.translation.y += delta.y;
            }
            *last_pos = Some(cursor);
        }
    } else {
        *last_pos = None;
    }
}

pub fn bird_slingshot(
    mut q_bird: Query<(&mut Transform, &mut Velocity, &mut ExternalImpulse), With<Bird>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut state: ResMut<BirdState>,
    mut gizmos: Gizmos,
) {
    let window = windows.single();
    let (camera, cam_transform) = q_camera.single();
    let Some((mut transform, mut vel, mut impulse)) = q_bird.iter_mut().next() else {
        return;
    };
    let cursor = window
        .cursor_position()
        .and_then(|c| camera.viewport_to_world_2d(cam_transform, c));
    if !state.launched {
        if let Some(cursor_pos) = cursor {
            let drag_vec = (cursor_pos - SLINGSHOT_ANCHOR).clamp_length_max(SLINGSHOT_MAX_DIST);
            if mouse.pressed(MouseButton::Left) {
                state.dragging = true;
                transform.translation.x = SLINGSHOT_ANCHOR.x + drag_vec.x;
                transform.translation.y = SLINGSHOT_ANCHOR.y + drag_vec.y;
                vel.linvel = Vec2::ZERO;
                impulse.impulse = Vec2::ZERO;
                let start = SLINGSHOT_ANCHOR.extend(10.0);
                let end = (SLINGSHOT_ANCHOR + -drag_vec).extend(10.0);
                gizmos.line(start, end, Color::YELLOW);
            } else if state.dragging && mouse.just_released(MouseButton::Left) {
                let launch_vec = -drag_vec;
                impulse.impulse = launch_vec * PIXEL_PHYSIC_SCALE;
                state.dragging = false;
                state.launched = true;
            }
        }
    }
}
