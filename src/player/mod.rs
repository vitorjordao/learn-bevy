use bevy::prelude::*;

use self::systems::*;

pub mod components;
mod systems;

// #[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
// pub struct MovementSystemSet;

// #[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
// pub struct ConfineSystemSet;

#[derive(Debug, SystemSet, Hash, PartialEq, Eq, Clone)]
pub enum PlayerSystemSet {
  Movement,
  Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
      app
        // .configure_set(Update, MovementSystemSet.before(ConfineSystemSet))
        .configure_set(Update, PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement.in_set(PlayerSystemSet::Movement))
        .add_systems(Update, confine_player_movement.in_set(PlayerSystemSet::Confinement))
        .add_systems(Update, confine_player_movement.after(player_movement))
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star);
  }
}