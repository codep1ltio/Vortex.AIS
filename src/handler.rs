mod server;
use server::*;

#[macro_use]
pub mod lib;
use lib::*;
use slint::{ModelRc, SharedString, VecModel};
use std::sync::{Mutex, OnceLock};

pub static LOGS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();
pub static UI: OnceLock<slint::Weak<MainWindow>> = OnceLock::new();
pub struct Launcher {
    ui: MainWindow,
}

impl Launcher {
    pub fn new() -> Self {
        LOGS.set(Mutex::new(vec![
            "Welcome to Vortex AIS".to_string(),
            "The console is here for us, and you to debug. If something goes wrong, console should hint the issue and you can ask our developers about it.".to_string()
        ])).ok();

        Self {
            ui: MainWindow::new().unwrap(),
        }
    }

    pub fn setup(&self) {
        UI.set(self.ui.as_weak()).ok();
        if let Some(logs) = LOGS.get() {
            let logs = logs.lock().unwrap();

            self.ui
                .global::<AppState>()
                .set_logs(ModelRc::new(VecModel::from(
                    logs.iter().map(SharedString::from).collect::<Vec<_>>(),
                )));
        }

        let weak = self.ui.as_weak();

        self.ui.global::<AppState>().on_FNnavigate(move |c| {
            let ui = weak.unwrap();
            ui.global::<AppState>().set_current_page(c);

            match ui.global::<AppState>().get_current_page().as_str() {
                "mods" => match load_mods() {
                    Some(Ok(mods)) => ui.global::<AppState>().set_mod_list(mods),
                    Some(Err(err)) => eprintln!("{err}"),

                    None => err_log!("Couldn't find mods."),
                },
                "home" => {
                    work_in_progress!();
                    err_log!("home page");
                }

                _ => todo!(),
            }
        });
    }

    pub fn run(&self) {
        self.ui.run().unwrap();
    }
}

pub fn push_log(text: String) {
    if let Some(logs) = LOGS.get() {
        let mut logs = logs.lock().unwrap();

        logs.push(text);

        if let Some(ui_weak) = UI.get()
            && let Some(ui) = ui_weak.upgrade()
        {
            ui.global::<AppState>()
                .set_logs(ModelRc::new(VecModel::from(
                    logs.iter()
                        .map(SharedString::from)
                        .collect::<Vec<SharedString>>(),
                )));
        }
    }
}
