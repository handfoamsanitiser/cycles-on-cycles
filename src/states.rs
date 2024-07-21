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


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MAIN_MENU,
    GAME,
    END
}

