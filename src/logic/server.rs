//slint::include_modules!();
use slint::ModelRc;

mod mod_loader;
use mod_loader::*;

use crate::logic::lib::*;

static MOD_FILE: &str = "mods.json";

pub fn load_mods() -> Result<ModelRc<ModList>, Box<dyn std::error::Error + Send + Sync>> {
    load_mods_json(MOD_FILE)
}
