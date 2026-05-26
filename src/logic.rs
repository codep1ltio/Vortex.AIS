mod server;
use server::*;

#[macro_export]
macro_rules! time {
    ($name:expr, $block:block) => {{
        let time = std::time::Instant::now();
        let result = $block;
        let dur = time.elapsed();
        println!(
            ">Task: \x1b[32m{}\x1b[0m\n>Time: \x1b[34m{:.1?}\x1b[0m",
            $name, dur
        );
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
