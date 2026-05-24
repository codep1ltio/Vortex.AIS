mod server;
use server::*;

#[macro_export]
macro_rules! analysis {
    ($name:expr, $block:block) => {{
        let time = std::time::Instant::now();
        let result = $block;
        let dur = time.elapsed();
        println!(">object: {}\n>time: {:.1?}", $name, dur);
        result
    }};
}

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

            if ui.global::<AppState>().get_current_page() == "mods" {
                match get_mods() {
                    Ok(mods) => ui.global::<AppState>().set_mod_list(mods),
                    Err(err) => eprintln!("{err}")
                }
            } else {
                // # TODO: Cache mods
            }
        });
    }

    pub fn run(&self) {
        analysis!("active", { self.ui.run().unwrap(); })
    }
}