use bevy::math::Vec2;
use uuid::Uuid;

use crate::texture::DynamicTexture;

pub struct User {
    username: String,
    type_: UserType,
    is_admin: bool,
}

pub enum UserType {
    Player(PlayerModel),
    Spectator,
}

pub struct PlayerModel {
    texture: DynamicTexture,
    location: Vec2,
    inventory: Vec<Item>,
    health: f32,
}

pub struct Item {
    location: Vec2,
    texture: DynamicTexture,
    cost: Option<u32>,
}
