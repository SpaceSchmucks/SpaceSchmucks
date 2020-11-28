use bevy::prelude::AppBuilder;
use bevy::prelude::Plugin;
use rhai::OptimizationLevel;
use rhai::{export_module, exported_module, plugin::*};

pub struct EnginePlugin;

impl Plugin for EnginePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(build_engine(None));
    }
}

#[export_module]
mod utils {
    // pub fn get_tile
}

pub fn build_engine(level_script: Option<&str>) -> anyhow::Result<Engine> {
    let mut engine = Engine::new();
    engine.set_optimization_level(OptimizationLevel::Full);

    let utils_module = exported_module!(utils);
    engine.load_package(utils_module);

    if let Some(script) = level_script {
        engine.consume(script)?;
    }

    Ok(engine)
}
