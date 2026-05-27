use slint::ModelRc;

mod mod_loader;
use mod_loader::*;

use crate::handler::lib::*;

static MOD_FILE: &str = "mods.json";

pub fn load_mods() -> Option<Result<ModelRc<ModList>, Box<dyn std::error::Error + Send + Sync>>> {
    Some(load_mods_json(MOD_FILE))
}
