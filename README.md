# Cortex

[![OSS by Subtale](https://img.shields.io/badge/oss_by-subtale-white?style=flat-square&labelColor=14213D&color=E5E5E5)][![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsubtalegames%2Fcortex.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsubtalegames%2Fcortex?ref=badge_shield)
[oss]
[![Chat on Discord](https://img.shields.io/badge/chat_on-discord-white?style=flat-square&labelColor=14213D&color=E5E5E5)][discord]
[![Crates.io](https://img.shields.io/crates/v/subtale-cortex?style=flat-square&label=latest&labelColor=14213D&color=E5E5E5)][crate]
[![MIT License](https://img.shields.io/badge/license-MIT-brightgreen?style=flat-square&labelColor=14213D&color=E5E5E5)][mit]
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-brightgreen?style=flat-square&labelColor=14213D&color=E5E5E5)][apache]

> Cortex is a flexible crash-handling solution for applications written in Rust.

## Example

```rs
use subtale_cortex::CrashHandler;

fn run_application() {
    // Your application logic...
}

fn crash_handler(output: std::process::Output) -> Result<(), Box<dyn std::error::Error>> {
    // Handle the output of the application process when it crashes
}

fn main() {
    let result = CrashHandler::with_process(run_application)
        .crash_handler(crash_handler)
        // Use `RUST_BACKTRACE` full in your application process
        .full_backtrace()
        .run();

    match result {
        // The application process finished successfully
        Ok(true) => ..,
        // The application process crashed, but the error was
        // handled successfully
        Ok(false) => ..,
        // An error was encountered spawning the application or
        // when crash handling
        Err(e) => ..,
    }
}
```

The [examples](examples/) directory has specific implementation examples, including using the [native-dialog][native-dialog] for cross-platform message boxes and running a [Bevy][bevy] game within the crash handler.

## How it works

Cortex uses the crash handling approach described in [this blog post][inspiration] by Mason Remaley (Anthropic Studios) from March 2021.

The crash handling implementation is simple and straightforward: when the application is launched, invoke the application again as a subprocess of itself and monitor the subprocess for non-successful exit codes.

To prevent the application from recursively invoking itself until infinity, a command argument (`--cortex-child`) is used to identify whether the process is the crash handler (and so a subprocess should be invoked) or a subprocess.

> For example, the first time that the application is run, Cortex identifies that the `--cortex-child` argument is not present. The application is then self-spawned as a subprocess, this time with the `--cortex-child` argument included
> so the regular application logic (`run_application()` in the example above) can start.

## License

Cortex is free and open source. Unless explicitly noted otherwise, all code in this repository is dual-licensed under the [MIT License][mit] and [Apache License, Version 2.0][apache].

This licensing approach is the de facto standard within the Rust ecosystem.


[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsubtalegames%2Fcortex.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsubtalegames%2Fcortex?ref=badge_large)

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[oss]: https://subtale.dev
[discord]: https://discord.subtale.com
[crate]: https://crates.io/crates/subtale-cortex
[mit]: LICENSE-MIT
[apache]: LICENSE-APACHE
[inspiration]: https://web.archive.org/web/20230324021914/https://www.anthropicstudios.com/2021/03/05/crash-reporter/
[native-dialog]: https://github.com/native-dialog-rs/native-dialog-rs
[bevy]: https://github.com/bevyengine/bevy