use native_dialog::{MessageDialog, MessageType};
use subtale_cortex::CrashHandler;

fn run_application() {
    panic!("Uh oh, something broke!");
}

fn crash_handler(output: std::process::Output) -> Result<(), Box<dyn std::error::Error>> {
    let report = MessageDialog::new()
        .set_title("Application has crashed!")
        .set_text("The application encountered a fatal exception and had to terminate.\n\nWould you like to report the crash to the developers?")
        .set_type(MessageType::Error)
        .show_confirm()?;

    if report {
        // The user wants to report the crash, so do something with
        // the output variable (dump to file, send over HTTPS, etc.)
    }

    Ok(())
}

fn main() {
    let _ = CrashHandler::with_process(run_application)
        .crash_handler(crash_handler)
        .run();
}
