use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Default)]
pub struct BirdState {
    pub dragging: bool,
    pub launched: bool,
}

#[derive(Resource, Default, Debug)]
pub struct PhysicsState {
    pub hash: String,
    pub step: u64,
}

#[derive(Resource)]
pub struct BirdStart(pub Vec2);

impl Default for BirdStart {
    fn default() -> Self {
        BirdStart(Vec2::new(0.0, 0.0))
    }
}
