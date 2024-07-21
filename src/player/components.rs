use bevy::prelude::*;


#[derive(Component)]
pub struct Player {}


#[derive(Component)]
pub enum DirectionX {
    RIGHT,
    LEFT
}


#[derive(Component, PartialEq)]
pub enum PlayerState {
    STANDING,
    MOVING,
    DEAD
}