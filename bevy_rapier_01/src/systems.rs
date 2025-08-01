use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::game::game_setup;
use crate::level::LevelData;
use crate::resources::*;

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
    bird_start: ResMut<BirdStart>,
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
        commands.remove_resource::<LevelData>();

        game_setup(commands, meshes, materials, bird_start);
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
    bird_start: Res<BirdStart>,
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
            let slingshot_anchor = bird_start.0;
            let drag_vec = (cursor_pos - slingshot_anchor).clamp_length_max(SLINGSHOT_MAX_DIST);
            if mouse.pressed(MouseButton::Left) {
                state.dragging = true;
                transform.translation.x = slingshot_anchor.x + drag_vec.x;
                transform.translation.y = slingshot_anchor.y + drag_vec.y;
                vel.linvel = Vec2::ZERO;
                impulse.impulse = Vec2::ZERO;
                let start = slingshot_anchor.extend(10.0);
                let end = (slingshot_anchor + -drag_vec).extend(10.0);
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

pub fn draw_grid(mut gizmos: Gizmos, level_data: Option<Res<LevelData>>) {
    if let Some(level_data) = level_data {
        let world_bound = level_data.world_bound;
        let world_width = (world_bound[2] - world_bound[0]) as f32;
        let world_height = (world_bound[3] - world_bound[1]) as f32;
        let grid_color = Color::rgba(0.25, 0.25, 0.25, 0.25);
        let cell_size = 50.0;
        let z = 0.0;
        for i in 0..=(world_width / cell_size) as u32 {
            let x = i as f32 * cell_size;
            gizmos.line(
                Vec3::new(x, 0.0, z),
                Vec3::new(x, -world_height, z),
                grid_color,
            );
        }
        for i in 0..=(world_height / cell_size) as u32 {
            let y = i as f32 * cell_size;
            gizmos.line(
                Vec3::new(0.0, -y, z),
                Vec3::new(world_width, -y, z),
                grid_color,
            );
        }
    }
}
