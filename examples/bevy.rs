use bevy::prelude::*;
use subtale_cortex::CrashHandler;

fn run_game() {
    let mut app = App::new();

    app.add_systems(Startup, hello_world);

    app.run();
}

fn hello_world() {
    info!("Hello world from Bevy!");
}

fn crash_handler(output: std::process::Output) -> Result<(), Box<dyn std::error::Error>> {
    // Here you would handle the process output that resulted in
    // the game crashing.

    Ok(())
}

fn main() {
    let _ = CrashHandler::with_process(run_game)
        .crash_handler(crash_handler)
        .run();
}
