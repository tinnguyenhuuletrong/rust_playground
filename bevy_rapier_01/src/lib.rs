use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::sprite::ColorMaterial;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Component)]
struct SelectableBox;
#[derive(Component)]
struct Bird;

#[derive(Resource, Default)]
struct BirdState {
    dragging: bool,
    launched: bool,
}

#[derive(Resource, Default, Debug)]
struct PhysicsState {
    hash: String,
    step: u64,
}

#[derive(Component)]
struct PhysicsStateText;

const BIRD_START: Vec2 = Vec2::new(-350.0, -120.0);
const BIRD_RADIUS: f32 = 14.0;
const SLINGSHOT_ANCHOR: Vec2 = Vec2::new(-350.0, BIRD_START.y);
const SLINGSHOT_MAX_DIST: f32 = 120.0;
const PIXEL_PHYSIC_SCALE: f32 = 10_000.0; // 100 * 100

pub fn bevy_main_app() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&"bevy_main_app() called in WASM".into());
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(DefaultPlugins);

    app.insert_resource::<RapierConfiguration>(RapierConfiguration {
        timestep_mode: TimestepMode::Fixed {
            dt: 1.0 / 30.0,
            substeps: 1,
        },
        ..Default::default()
    });

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default().in_fixed_schedule())
        .add_plugins(RapierDebugRenderPlugin::default())
        .init_resource::<BirdState>()
        .init_resource::<PhysicsState>()
        .add_systems(Startup, (game_setup, ui_setup))
        .add_systems(Update, (camera_drag, bird_slingshot, game_state_control))
        .add_systems(
            FixedUpdate,
            (physic_step_hash_check, ui_update_physic_state_text),
        )
        .run();
}

fn ui_setup(mut commands: Commands) {
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

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // By removing and re-adding the RapierContext, we are ensuring that the
    // physics simulation is completely reset.
    commands.remove_resource::<RapierContext>();
    commands.insert_resource(RapierContext::default());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0),
        ..Default::default()
    });
    let ground_color = Color::rgb(0.3, 0.3, 0.3);
    let box_color = Color::rgb(0.7, 0.4, 0.2);
    let ground_size = Vec2::new(1_000.0, 20.0);
    // Move ground down a bit
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ground_color,
                custom_size: Some(ground_size),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, -170.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(ground_size.x / 2.0, ground_size.y / 2.0),
    ));

    // Add left wall
    let wall_thickness = 20.0;
    let wall_height = 600.0;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ground_color,
                custom_size: Some(Vec2::new(wall_thickness, wall_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-500.0, 120.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0),
        Restitution::coefficient(0.8),
    ));
    // Add right wall
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ground_color,
                custom_size: Some(Vec2::new(wall_thickness, wall_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(500.0, 120.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0),
        Restitution::coefficient(0.8),
    ));
    let box_size = 24.0;
    let rows = 5;
    for row in 0..rows {
        let num_boxes = 4;
        let y = -70.0 + (row as f32) * (box_size + 10.0);
        let x_start = 120.0;
        for i in 0..num_boxes {
            let x = x_start + i as f32 * (box_size + 10.0);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: box_color,
                        custom_size: Some(Vec2::splat(box_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x, y, 1.0),
                    ..Default::default()
                },
                RigidBody::Dynamic,
                Collider::cuboid(box_size / 2.0, box_size / 2.0),
                Restitution::coefficient(0.6),
                SelectableBox,
            ));
        }
    }
    let mesh = meshes.add(Mesh::from(Circle::new(BIRD_RADIUS)));
    let material = materials.add(ColorMaterial::from(Color::rgb(0.9, 0.1, 0.1)));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: mesh.into(),
            material,
            transform: Transform::from_xyz(BIRD_START.x, BIRD_START.y, 2.0),
            ..Default::default()
        },
        RigidBody::Dynamic,
        Collider::ball(BIRD_RADIUS),
        Restitution::coefficient(0.6),
        Damping {
            linear_damping: 0.2,
            angular_damping: 0.2,
        },
        Bird,
        Velocity::zero(),
        ExternalImpulse::default(),
    ));
}

fn game_state_control(
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

fn camera_drag(
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

fn bird_slingshot(
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

fn physic_step_hash_check(
    rapier_context: Res<RapierContext>,
    rapier_config: Res<RapierConfiguration>,
    q_dynamic_bodies: Query<&Transform, With<RigidBody>>,
    mut physic_state: ResMut<PhysicsState>,
) {
    if !rapier_config.physics_pipeline_active {
        return;
    }
    if rapier_context.bodies.len() <= 0 {
        return;
    }

    if rapier_context.bodies.iter().all(|(_, b)| b.is_sleeping()) {
        if !physic_state.hash.is_empty() {
            return;
        }
        let mut state_str = String::new();
        for transform in q_dynamic_bodies.iter() {
            state_str.push_str(&format!(
                "T:{:.3}, R:{:.3}",
                transform.translation, transform.rotation
            ));
        }
        let mut hasher = Sha256::new();
        hasher.update(state_str.as_bytes());
        let hash = hasher.finalize();
        physic_state.hash = format!("{:x}", hash);
    } else {
        physic_state.step += 1;
    }
}

fn ui_update_physic_state_text(
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

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn start() {
        bevy_main_app();
    }
}
