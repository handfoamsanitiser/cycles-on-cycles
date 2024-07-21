use bevy::prelude::*;


pub mod components;
pub mod systems;

use systems::*;


pub struct MySpriteAnimationPlugin;

impl Plugin for MySpriteAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, animate_sprite);
    }
}