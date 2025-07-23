use bevy::math::primitives::Circle;
use bevy::prelude::*;
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

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    let level_data = load_level_data();

    let ground_color = Color::rgb(0.3, 0.3, 0.3);
    let ground_size = Vec2::new(level_data.world_size[0] as f32, 20.0);
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
    let wall_height = level_data.world_size[1] as f32;
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: ground_color,
                custom_size: Some(Vec2::new(wall_thickness, wall_height)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-(level_data.world_size[0] as f32) / 2.0, 120.0, 0.0),
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
            transform: Transform::from_xyz((level_data.world_size[0] as f32) / 2.0, 120.0, 0.0),
            ..Default::default()
        },
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0),
        Restitution::coefficient(0.8),
    ));

    let box_color = Color::rgb(0.7, 0.4, 0.2);
    let box_size = 24.0;

    for entity in level_data.entities {
        let position = Vec2::new(entity.position[0] as f32, entity.position[1] as f32);
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
}

fn load_level_data() -> LevelData {
    // Version read from file
    // let mut file = File::open("assets/data/level_01.json").expect("Failed to open level file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Failed to read level file");

    // Use embeded instead
    let bytes = include_bytes!("../assets/data/level_01.json");
    let contents = str::from_utf8(bytes).unwrap();
    let level_data: LevelData =
        serde_json::from_str(&contents).expect("Failed to parse level file");
    level_data
}
