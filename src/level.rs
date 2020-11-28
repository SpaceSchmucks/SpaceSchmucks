use std::io::{Read, Seek};

use bevy::prelude::Assets;
use rhai::Engine;
use zip::ZipArchive;

use crate::engine::build_engine;
use crate::tile::TileMap;

pub struct Level {
    pub engine: Engine,
    pub map: TileMap,
    pub tiles: Assets,
}

impl Level {
    pub fn load_from_archive<R: Read + Seek>(archive: ZipArchive<R>) -> anyhow::Result<Self> {
        let engine = build_engine(Some(Self::read_from_archive(archive, "main.rhai")?));
    }

    fn read_from_archive<R: Read + Seek>(
        archive: ZipArchive<R>,
        file: &str,
    ) -> anyhow::Result<String> {
        let file = archive.by_name(file)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(buffer)
    }

    // fn load_map<
}
