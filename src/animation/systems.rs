use bevy::prelude::*;

use super::components::*;


pub fn animate_sprite(
    mut query: Query<(&mut TextureAtlas, &SpriteAnimationIndices, &mut SpriteAnimationTimer)>,
    time: Res<Time>,
) {
    for (mut texture_atlas, sprite_animation_indeces, mut sprite_animation_timer) in query.iter_mut() {
        sprite_animation_timer.timer.tick(time.delta());
        if sprite_animation_timer.timer.just_finished() {
            if texture_atlas.index == sprite_animation_indeces.end {
                texture_atlas.index = sprite_animation_indeces.start;
            } else {
                texture_atlas.index += 1;
            }
        }
    }
}