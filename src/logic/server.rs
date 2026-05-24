slint::include_modules!();
use slint::{ModelRc, VecModel};
use std::rc::Rc;

pub fn get_mods() -> Result<ModelRc<ModList>, String> {
    let template = ModList {
        mods: ModelRc::from(Rc::new(VecModel::from(vec![Mod {
            name: "test".into(),
            author: "cod".into(),
            description: "cool addon".into(),
            version: "1.0.0".into(),
            id: 1,
        }]))),
    };

    Ok(ModelRc::from(Rc::new(VecModel::from(vec![template]))))
}
