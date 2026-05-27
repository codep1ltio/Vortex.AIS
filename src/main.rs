mod handler;
use handler::Launcher;

fn boot() -> Result<(), String> {
    let boot = Launcher::new();
    boot.setup();
    boot.run();

    Ok(())
}

fn main() {
    if let Err(err) = boot() {
        println!("{err}");
        //std::process::exit(1);
    }
}
