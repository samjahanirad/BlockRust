use bevy::prelude::*;

#[derive(Component)]
struct Tile {
    texture_id: String,
}

#[derive(Component)]
struct Player {
    position: Vec2,
}
