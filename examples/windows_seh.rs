use subtale_cortex::CrashHandler;

fn run_application() {
    println!("Running application!");
    unsafe {
        *(0 as *mut i32) = 42;
    }
}

fn main() { let _ = CrashHandler::with_process(run_application).run(); }
