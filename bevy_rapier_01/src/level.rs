
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Resource)]
pub struct LevelData {
    pub world_bound: [i32; 4], // left, top, right, bottom
    pub ground_y: i32,
    pub entities: Vec<EntityData>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct EntityData {
    pub entity_type: String,
    pub position: [i32; 2],
}
