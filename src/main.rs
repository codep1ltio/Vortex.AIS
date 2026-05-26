mod logic;
use logic::Launcher;

fn boot() -> Result<(), String> {
    let boot = Launcher::new();
    boot.setup_callbacks();
    boot.run();

    Ok(())
}

fn main() {
    if let Err(err) = boot() {
        println!("{err}");
        //std::process::exit(1);
    }
}
