use bevy::prelude::Resource;

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
