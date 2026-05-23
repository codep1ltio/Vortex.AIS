slint::include_modules!();

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
        });
    }

    pub fn run(&self) {
        self.ui.run().unwrap();
    }
}