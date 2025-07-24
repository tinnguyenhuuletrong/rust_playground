use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::render::mesh::Mesh;
use bevy::sprite::ColorMaterial;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::level::LevelData;
use crate::resources::BirdStart;

const BIRD_RADIUS: f32 = 14.0;

pub fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bird_start: ResMut<BirdStart>,
) {
    // By removing and re-adding the RapierContext, we are ensuring that the
    // physics simulation is completely reset.
    commands.remove_resource::<RapierContext>();
    commands.insert_resource(RapierContext::default());

    let level_data = load_level_data();
    let world_bound = level_data.world_bound;
    let world_width = (world_bound[2] - world_bound[0]) as f32;
    let world_height = (world_bound[3] - world_bound[1]) as f32;

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(world_height);
    camera_bundle.transform.translation.x = world_width / 2.0;
    camera_bundle.transform.translation.y = -world_height / 2.0;
    commands.spawn(camera_bundle);

    let ground_y = level_data.ground_y as f32;
    let ground_height = world_height - ground_y;
    let ground_color = Color::rgb(0.3, 0.3, 0.3);
    let ground_size = Vec2::new(world_width, ground_height);

    let y_translation = -ground_y - (ground_height / 2.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ground_color,
                custom_size: Some(ground_size),
                ..Default::default()
            },
            transform: Transform::from_xyz(world_width / 2.0, y_translation, 0.0),
            ..Default::default()
        },
        Collider::cuboid(ground_size.x / 2.0, ground_size.y / 2.0),
    ));

    // Add left wall
    let wall_thickness = 50.0;
    let wall_height = world_height;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ground_color,
                custom_size: Some(Vec2::new(wall_thickness, wall_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(wall_thickness / 2.0, -wall_height / 2.0, 0.0),
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
            transform: Transform::from_xyz(
                world_width - wall_thickness / 2.0,
                -wall_height / 2.0,
                0.0,
            ),
            ..Default::default()
        },
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0),
        Restitution::coefficient(0.8),
    ));

    let box_color = Color::rgb(0.7, 0.4, 0.2);
    let box_size = 24.0;

    for entity in &level_data.entities {
        let position = Vec2::new(entity.position[0] as f32, -(entity.position[1] as f32));
        match entity.entity_type.as_str() {
            "box" => {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: box_color,
                            custom_size: Some(Vec2::splat(box_size)),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(position.x, position.y, 1.0),
                        ..Default::default()
                    },
                    RigidBody::Dynamic,
                    Collider::cuboid(box_size / 2.0, box_size / 2.0),
                    Restitution::coefficient(0.6),
                    SelectableBox,
                ));
            }
            "bird" => {
                bird_start.0 = position;
                let mesh = meshes.add(Mesh::from(Circle::new(BIRD_RADIUS)));
                let material = materials.add(ColorMaterial::from(Color::rgb(0.9, 0.1, 0.1)));
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: mesh.into(),
                        material,
                        transform: Transform::from_xyz(position.x, position.y, 2.0),
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
            _ => {}
        }
    }
    commands.insert_resource(level_data);
}

fn load_level_data() -> LevelData {
    // Version read from file
    // let mut file = File::open("assets/data/level_01.json").expect("Failed to open level file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Failed to read level file");

    // Use embeded instead
    let bytes = include_bytes!("../assets/data/level_01.json");
    let contents = std::str::from_utf8(bytes).unwrap();
    let level_data: LevelData =
        serde_json::from_str(&contents).expect("Failed to parse level file");
    level_data
}
