/// A wrapper around a process closure that handles crashes by running the
/// closure as a subprocess and invoking a crash handler closure if the
/// subprocess fails.
pub struct CrashHandler {
    /// Closure that runs a process.
    process_closure: Box<dyn Fn()>,
    /// Command line flag that identifies a child process (and prevents infinite
    /// recursion of spawning subprocesses).
    child_flag: String,
    /// Closure that handles crashes, accepting the output of the subprocess.
    crash_handler_closure:
        Box<dyn Fn(std::process::Output) -> Result<(), Box<dyn std::error::Error>> + 'static>,
    /// The value of the `RUST_BACKTRACE` environment variable, to be set in the
    /// subprocess.
    backtrace: Option<&'static str>,
}

impl Default for CrashHandler {
    /// Create a new crash reporter with default settings (a process that prints
    /// "Hello, world!", "--cortex-bypass" bypass flag, a crash
    /// handler that prints the status code and error message using
    /// `eprintln!`, and `RUST_BACKTRACE=full`).
    fn default() -> Self {
        Self {
            process_closure: Box::new(|| println!("Hello, world!")),
            child_flag: "--cortex-child".to_string(),
            crash_handler_closure: Box::new(|output| {
                let status = output.status.code().unwrap_or(-1);
                let error = String::from_utf8_lossy(&output.stderr);

                eprintln!("Status: {status}\nError: {error}");
                Ok(())
            }),
            backtrace: None,
        }
    }
}

impl CrashHandler {
    /// Create a new crash reporter with default settings.
    pub fn new() -> Self { Self::default() }

    /// Create a new crash reporter from the given closure that runs a process.
    pub fn with_process(process: impl Fn() + 'static) -> Self {
        Self {
            process_closure: Box::new(process),
            ..Default::default()
        }
    }

    /// Sets the command line flag that identifies a child process.
    pub fn child_flag(mut self, flag: impl Into<String>) -> Self {
        self.child_flag = flag.into();
        self
    }

    /// Sets the crash handler that is called when the process fails.
    pub fn crash_handler(
        mut self,
        crash_handler: impl Fn(std::process::Output) -> Result<(), Box<dyn std::error::Error>> + 'static,
    ) -> Self {
        self.crash_handler_closure = Box::new(crash_handler);
        self
    }

    /// Sets the value of the `RUST_BACKTRACE` environment variable in the
    /// subprocess to `1`.
    pub fn backtrace(mut self) -> Self {
        self.backtrace = Some("1");
        self
    }

    /// Sets the value of the `RUST_BACKTRACE` environment variable in the
    /// subprocess to `full`.
    pub fn full_backtrace(mut self) -> Self {
        self.backtrace = Some("full");
        self
    }

    /// Runs the configured process as a subprocess and handle crashes if the
    /// child flag is not present, otherwise run the process normally.
    ///
    /// # Returns
    /// - `Ok(true)` if the process ran successfully without errors.
    /// - `Ok(false)` if there was an error in the process (that was handled).
    /// - `Err` if there was an error spawning the process or handling an error
    ///   in the process.
    pub fn run(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // Capture the current CLI arguments
        let mut args = std::env::args().collect::<Vec<_>>();

        // If the child flag is present, run the process normally
        if args.contains(&self.child_flag) {
            (self.process_closure)();
        } else {
            // Remove the first argument (path to executable) and add the child flag
            args.remove(0);
            args.push(self.child_flag.clone());

            // Spawn current exe as subprocess and read process output
            let output = std::process::Command::new(std::env::current_exe()?)
                // Passthrough the current arguments
                .args(args)
                // Passthrough the current environment
                .envs(std::env::vars())
                // Set the RUST_BACKTRACE environment variable if configured
                .env("RUST_BACKTRACE", self.backtrace.unwrap_or("0"))
                // Spawn the subprocess and capture its output
                .output()?;

            // If the subprocess failed, call the crash handler closure
            if !output.status.success() {
                (self.crash_handler_closure)(output)?;
                return Ok(false);
            }
        }

        Ok(true)
    }
}
