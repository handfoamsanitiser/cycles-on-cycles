use bevy::prelude::*;


pub mod systems;

use systems::*;


pub struct DevToolPlugin;


impl Plugin for DevToolPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (to_game, to_end, to_main_menu));
    }
} 