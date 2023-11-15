use bevy::prelude::*;

use self::{resources::EnemySpawnTimer, systems::*};

pub mod components;
mod systems;
pub mod resources;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {

    fn build(&self, app: &mut App) {
        app
            .init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, update_enemy_direction)
            .add_systems(Update, confine_enemy_movement)
            .add_systems(Update, tick_enemy_spawn_timer)
            .add_systems(Update, spawn_enemy_over_time);
    }
}