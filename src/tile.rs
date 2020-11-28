use bevy::math::Vec2;
use bevy::prelude::Texture;
use bevy::utils::HashMap;

#[derive(Default)]
pub struct TileMap {
    pub tiles: HashMap<Vec2, Tile>,
}

impl TileMap {
    pub fn from_iter(tile_iter: impl Iterator<Item = (Vec2, Tile)>) -> Self {
        TileMap {
            tiles: tile_iter.collect(),
        }
    }

    pub fn get_tile<'m>(&'m self, pos: Vec2) -> Option<&'m Tile> {
        self.tiles.get(pos)
    }

    pub fn get_tile_mut<'m>(&'m mut self, pos: Vec2) -> Option<&'m mut Tile> {
        self.tiles.get_mut(pos)
    }

    /// Iterates over the tiles visible from the given position
    pub fn visible_from<'m>(&'m self, pos: Vec2) -> impl Iterator<(Vec2, &'m Tile)> {
        unimplemented!()
    }
}

/// A floor tile
pub struct Tile {
    pub texture: Texture,
    pub layers: Vec<TileEffect>,
    pub wall: Option<Wall>,
}

impl Tile {
    pub fn new(texture: Texture, layers: Vec<TileEffect>, wall: Option<Wall>) -> Tile {
        Tile {
            texture,
            layers,
            wall,
        }
    }
}

/// An effect rendered on top of a tile
pub struct TileEffect {
    pub texture: Texture,
    pub effect_id: String,
}

impl TileEffect {
    pub fn new(texture: Texture, effect_id: String) -> Self {
        TileEffect { texture, effect_id }
    }
}

/// A wall on top of a tile that impedes the movement of entities
pub struct Wall {
    pub texture: Texture,
    pub passable: bool,
    pub wall_type_id: String,
}

impl Wall {
    pub fn new(texture: Texture, passable: bool, wall_type_id: String) -> Self {
        Self {
            texture,
            passable,
            wall_type_id,
        }
    }
}
