mod logic;
use logic::Launcher;

fn main() {
    let boot = Launcher::new();
    boot.setup_callbacks();
    boot.run();
}