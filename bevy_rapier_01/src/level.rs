
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LevelData {
    pub world_size: [u32; 2],
    pub entities: Vec<EntityData>,
}

#[derive(Deserialize)]
pub struct EntityData {
    pub entity_type: String,
    pub position: [i32; 2],
}
