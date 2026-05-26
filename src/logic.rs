mod server;
use server::*;

pub mod lib;

pub struct Launcher {
    ui: MainWindow,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            ui: MainWindow::new().unwrap(),
        }
    }

    pub fn setup_callbacks(&self) {
        let weak = self.ui.as_weak();

        self.ui.global::<AppState>().on_FNnavigate(move |c| {
            let ui = weak.unwrap();
            ui.global::<AppState>().set_current_page(c);

            match ui.global::<AppState>().get_current_page().as_str() {
                "mods" => match load_mods() {
                    Ok(mods) => ui.global::<AppState>().set_mod_list(mods),
                    Err(err) => eprintln!("{err}"),
                },
                _ => todo!(),
            }
        });
    }

    pub fn run(&self) {
        self.ui.run().unwrap();
    }
}
