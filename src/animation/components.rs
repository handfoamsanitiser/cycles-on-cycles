use bevy::prelude::*;


#[derive(Component)]
pub struct SpriteAnimationIndices {
    pub start: usize,
    pub end: usize,
}

#[derive(Component)]
pub struct SpriteAnimationTimer {
    pub timer: Timer,
}