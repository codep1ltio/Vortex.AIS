use serde::Deserialize;
use slint::{ModelRc, VecModel};
use std::cell::RefCell;
use std::{fs, rc::Rc};

use crate::handler::lib::*;
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

thread_local! {
    static MOD_CACHE: RefCell<Option<ModelRc<ModList>>> = const { RefCell::new(None) };
}

pub fn load_mods_json(
    file: &str,
) -> Result<ModelRc<ModList>, Box<dyn std::error::Error + Send + Sync>> {
    if let Some(cached) = MOD_CACHE.with(|c| c.borrow().clone()) {
        return Ok(cached);
    }
    time!("load mods", {
        let json = fs::read_to_string(file)?;
        let mut parsed: JsonModList = serde_json::from_str(&json)?;

        {
            //    Simple mod sorter, if favorites higher, it will move it up but if equal it will compare downloads then blablabla
            //    might turn this into a macro sometime later if i reuse this bcs its really useful

            parsed.mods.sort_by(|a, b| {
                b.favorites
                    .cmp(&a.favorites)
                    .then(b.downloads.cmp(&a.downloads))
            });

            parsed.mods.iter_mut().enumerate().for_each(|(i, m)| {
                m.id = (i + 1) as i32;
            });
        }

        let slint_mods = parsed
            .mods
            .into_iter()
            .map(|m| Mod {
                name: m.name.into(),
                author: m.author.into(),
                description: m.description.into(),
                version: m.version.into(),
                favorites: m.favorites,
                downloads: m.downloads,
                id: m.id,
            })
            .collect::<Vec<Mod>>();

        let model = ModelRc::from(Rc::new(VecModel::from(vec![ModList {
            mods: ModelRc::from(Rc::new(VecModel::from(slint_mods))),
        }])));

        MOD_CACHE.with(|c| {
            *c.borrow_mut() = Some(model.clone());
        });

        Ok(model)
    })
}
