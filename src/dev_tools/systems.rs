use bevy::prelude::*;

use crate::states::GameState;


pub fn to_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        next_game_state.set(GameState::GAME);
    }
}


pub fn to_end(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyE) {
        next_game_state.set(GameState::END);
    }
}


pub fn to_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_game_state: ResMut<NextState<GameState>>
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        next_game_state.set(GameState::MAIN_MENU);
    }
}
