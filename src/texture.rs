use bevy::prelude::Texture;

pub struct DynamicTexture {
    asset: Texture,
    visible: bool,
    // TODO: allow dynamic rendering with WASM or rhai
}

impl DynamicTexture {
    pub fn new(asset: Texture) -> Self {
        Self {
            asset,
            visible: true,
        }
    }
}
