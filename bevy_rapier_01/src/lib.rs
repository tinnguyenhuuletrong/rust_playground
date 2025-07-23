use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod game;
mod level;
mod physics;
mod resources;
mod systems;
mod ui;

use game::game_setup;
use physics::physic_step_hash_check;
use resources::*;
use systems::{bird_slingshot, camera_drag, game_state_control};
use ui::{ui_setup, ui_update_physic_state_text};

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
        .init_resource::<BirdStart>()
        .add_systems(Startup, (game_setup, ui_setup))
        .add_systems(Update, (camera_drag, bird_slingshot, game_state_control))
        .add_systems(
            FixedUpdate,
            (physic_step_hash_check, ui_update_physic_state_text),
        )
        .run();
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
