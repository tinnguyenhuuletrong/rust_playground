use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use sha2::{Digest, Sha256};

use crate::resources::PhysicsState;

pub fn physic_step_hash_check(
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
