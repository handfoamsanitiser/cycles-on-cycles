use bevy::prelude::*;


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnState {
    #[default]
    PLAYER,
    ENEMY
}


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    #[default]
    RUNNING,
    WIN,
    LOSE
}

