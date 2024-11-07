use bevy::input::{mouse::MouseMotion, Input};
use bevy::prelude::*;

pub fn update_input(mut commands: Commands, mut player_query: Query<&mut Player>) {
    let (x, y) = Window::get_current_position().unwrap();

    if Input::keyboard().is_key_pressed(KeyCode::W) {
        player_query.first_mut().unwrap().position.y -= 1;
    }
    if Input::keyboard().is_key_pressed(KeyCode::S) {
        player_query.first_mut().unwrap().position.y += 1;
    }
    if Input::keyboard().is_key_pressed(KeyCode::A) {
        player_query.first_mut().unwrap().position.x -= 1;
    }
    if Input::keyboard().is_key_pressed(KeyCode::D) {
        player_query.first_mut().unwrap().position.x += 1;
    }

    if Input::keyboard().is_key_pressed(KeyCode::O) {
    }
}
