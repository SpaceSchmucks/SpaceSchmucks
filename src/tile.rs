use crate::texture::DynamicTexture;

pub struct Tile {
    texture: DynamicTexture,
    layers: Vec<TileLayer>,
    wall: Option<Wall>,
}

pub enum TileLayer {
    Object, // TODO: add fields for entity
    Effect, // TODO: create struct for effect layers
}

pub struct Wall {
    pub texture: DynamicTexture,
    pub passable: bool,
}

impl Wall {
    pub fn new(texture: DynamicTexture, passable: bool) -> Self {
        Self { texture, passable }
    }
}
