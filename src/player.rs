use anyhow::Context;
use bevy::math::Vec2;
use bevy::prelude::Texture;
use pgp::composed::key::SecretKeyParams;
use pgp::crypto::{HashAlgorithm, SymmetricKeyAlgorithm};
use pgp::types::CompressionAlgorithm;
use pgp::KeyType;
use pgp::SignedSecretKey;
use smallvec::smallvec;

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
    texture: Texture,
    location: Vec2,
    inventory: Vec<Item>,
    health: f32,
}

pub struct Item {
    location: Vec2,
    texture: Texture,
    cost: Option<u32>,
}

pub fn generate_key(username: String) -> anyhow::Result<SignedSecretKey> {
    let secret_key_params = SecretKeyParams::build()
        .key_type(KeyType::Rsa(2048))
        .can_create_certificates(false)
        .can_sign(true)
        .primary_user_id(username)
        .preferred_symmetric_algorithms(smallvec![SymmetricKeyAlgorithm::AES256])
        .preferred_hash_algorithms(smallvec![HashAlgorithm::SHA2_256])
        .preferred_compression_algorithms(smallvec![CompressionAlgorithm::ZLIB])
        .build()
        .context("Must be able to create secret key params")?;
    let secret_key = secret_key_params
        .generate()
        .context("Failed to generate a plain key")?;

    secret_key
        .sign(|| String::new())
        .context("Must be able to sign its own metadata")
}
