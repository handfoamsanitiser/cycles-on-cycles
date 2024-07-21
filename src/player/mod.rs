use bevy::prelude::*;

mod components;
mod systems;

use systems::*;


pub const PLAYER_SPEED: f32 = 0.1;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (set_player_target_pos, player_hit_wall, move_player, stop_player).chain());
    }
}