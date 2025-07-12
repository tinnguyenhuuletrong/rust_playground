use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::sprite::ColorMaterial;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct SelectableBox;
#[derive(Component)]
pub struct Bird;

#[derive(Resource, Default)]
pub struct BirdState {
    pub dragging: bool,
    pub launched: bool,
}

pub const BIRD_START: Vec2 = Vec2::new(-350.0, -120.0);
pub const BIRD_RADIUS: f32 = 14.0;
pub const SLINGSHOT_ANCHOR: Vec2 = Vec2::new(-350.0, BIRD_START.y);
pub const SLINGSHOT_MAX_DIST: f32 = 120.0;

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

    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .init_resource::<BirdState>()
        .add_systems(Startup, (game_setup, ui_setup))
        .add_systems(Update, (camera_drag, bird_slingshot, game_state_control))
        .run();
}

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
}

pub fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
        let y = -70.0 + (row as f32) * (box_size + 2.0);
        let x_start = 120.0;
        for i in 0..num_boxes {
            let x = x_start + i as f32 * (box_size + 2.0);
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

pub fn game_state_control(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    q_boxes: Query<Entity, With<SelectableBox>>,
    q_bird: Query<Entity, With<Bird>>,
    q_camera: Query<Entity, With<Camera2d>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut bird_state: ResMut<BirdState>,
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
                impulse.impulse = launch_vec;
                state.dragging = false;
                state.launched = true;
            }
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
