use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::sprite::ColorMaterial;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;

use crate::components::*;

const BIRD_START: Vec2 = Vec2::new(-350.0, -120.0);
const BIRD_RADIUS: f32 = 14.0;

pub fn game_setup(
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
