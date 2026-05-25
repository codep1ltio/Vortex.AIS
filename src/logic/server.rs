slint::include_modules!();
use slint::{ModelRc, VecModel};

use serde::Deserialize;
use std::{fs,cell::RefCell,rc::Rc};

use crate::time;

#[derive(Deserialize)]
struct JsonMod {
    name: String,
    author: String,
    description: String,
    version: String,
    favorites: i32,
    downloads: i32,
    id: i32,
}

#[derive(Deserialize)]
struct JsonModList {
    mods: Vec<JsonMod>,
}

/*  
    25/5/2026 - @cod.io
    Incase this gets expanded by someone other than me (@cod.io), 
    i'm gonna explain the reasoning and understanding of code blocks under this a bit.

    if you don't know what thread_local! macro is, it's to assign a unique variable to 
    each value in each thread because slint is not thread safe if variable name wasn't 
    obvious enough, mod_cache is caching the loaded mods so we dont have to load them again.

    time! macro is in logic.rs, src/ directory, it's just to test performance, can be removed on release

    the reason the structs have Json infront is to avoid conflicting with 
    slint's generated code, since that's what they are called in the slint, ui files

    nvm the code is still self explanatory, im too lazy to explain, that's all, thank you <3
    might come back to this tho sometime later
*/

thread_local! {
    static MOD_CACHE: RefCell<Option<ModelRc<ModList>>> = RefCell::new(None);
}

static MOD_FILE: &str = "mods.json";

pub fn load_mods() -> Result<ModelRc<ModList>, Box<dyn std::error::Error>> {
    if let Some(cached) = MOD_CACHE.with(|c| c.borrow().clone()) {
        return Ok(cached);
    }
    time!("load mods", {
        let json = fs::read_to_string(MOD_FILE)?;
        let mut parsed: JsonModList = serde_json::from_str(&json)?;

        {
            //    Simple mod sorter, if favorites higher, it will move it up but if equal it will compare downloads then blablabla
            //    might turn this into a macro sometime later if i reuse this bcs its really useful
    
            parsed.mods.sort_by(|a, b| {
            b.favorites                     
                .cmp(&a.favorites)
                .then(b.downloads.cmp(&a.downloads))
            });

            for (i, m) in parsed.mods.iter_mut().enumerate() {
                m.id = (i + 1) as i32;
            }
        }

        let slint_mods = parsed.mods
            .into_iter()
            .map(|m| Mod {
                name: m.name.into(),
                author: m.author.into(),
                description: m.description.into(),
                version: m.version.into(),
                favorites: m.favorites.into(),
                downloads: m.downloads.into(),
                id: m.id,
            })
            .collect::<Vec<Mod>>();

        let model = ModelRc::from(Rc::new(VecModel::from(vec![
            ModList { 
                mods: ModelRc::from(Rc::new(VecModel::from(slint_mods))), 
            }
        ])));

        MOD_CACHE.with(|c| {
            *c.borrow_mut() = Some(model.clone());
        });

        Ok(model)
    })
}