pub mod events;
mod systems;

pub mod enemy;
mod player;
pub mod score;
pub mod star;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use std::time::Duration;

use bevy::{
    prelude::*,
    asset::ChangeWatcher,
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: Some(ChangeWatcher {
                delay: Duration::new(5, 0),
            }),
            ..Default::default()
        }))
        .add_plugins(EnemyPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(StarPlugin)
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .run() 
}

