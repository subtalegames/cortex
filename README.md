![Cortex GitHub Banner](https://github.com/subtalegames/cortex/assets/24438483/07c5bf20-b471-40cd-b2ac-7e4998a41232)

[![OSS by Subtale](https://img.shields.io/badge/oss_by-subtale-f0f0f1?style=flat-square&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPHN2ZyBpZD0iTGF5ZXJfMiIgZGF0YS1uYW1lPSJMYXllciAyIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI3MTQuOTciIGhlaWdodD0iNDYxLjEzIiB2aWV3Qm94PSIwIDAgNzE0Ljk3IDQ2MS4xMyI%2BCiAgPGcgaWQ9IkxheWVyXzEtMiIgZGF0YS1uYW1lPSJMYXllciAxIj4KICAgIDxnPgogICAgICA8cGF0aCBkPSJNMzU3LjQ4LDM1Ny41MWgtMjIyLjc4Yy05Ljk1LDAtMTkuNS0zLjk1LTI2LjU0LTEwLjk5TDAsMjM4LjM1aDIzOC4zMmwxMTkuMTYsMTE5LjE2WiIgZmlsbD0iI2ZmZiIgc3Ryb2tlLXdpZHRoPSIwIi8%2BCiAgICAgIDxwYXRoIGQ9Ik03MTQuOTcsMjM4LjM1bC0yMTEuNzgsMjExLjc4Yy0xNC42NiwxNC42Ni0zOC40MiwxNC42Ni01My4wOCwwbC05Mi42Mi05Mi42MiwxMTkuMTYtMTE5LjE2aDIzOC4zMloiIGZpbGw9IiNmZmYiIHN0cm9rZS13aWR0aD0iMCIvPgogICAgICA8cGF0aCBkPSJNNDc2LjY3LDExMC43M3YxNi45MWMtMjguMzEsMC01Ni42OCwxMC44Mi03OC4zMiwzMi40Ny0yMS42NSwyMS41OC0zMi40MSw0OS44OS0zMi40MSw3OC4yNmgtMTYuOTFjMC0yOC4zMS0xMC44Mi01Ni42OC0zMi40Ny03OC4zMi0yMS41OC0yMS42NS00OS44OS0zMi40LTc4LjI2LTMyLjR2LTE2LjkxYzI4LjMxLDAsNTYuNjgtMTAuODIsNzguMzItMzIuNDcsMjEuNjUtMjEuNTgsMzIuNDEtNDkuODksMzIuNDEtNzguMjZoMTYuOTFjMCwyOC4zMSwxMC44Miw1Ni42OCwzMi40Nyw3OC4zMiwyMS41OCwyMS42NSw0OS44OSwzMi40LDc4LjI2LDMyLjRaIiBmaWxsPSIjZmZmIiBzdHJva2Utd2lkdGg9IjAiLz4KICAgIDwvZz4KICA8L2c%2BCjwvc3ZnPg%3D%3D&logoColor=f0f0f1&labelColor=2060d3)][oss]
[![Chat on Discord](https://img.shields.io/badge/chat_on-discord-f0f0f1?style=flat-square&logo=discord&logoColor=f0f0f1&labelColor=2060d3)][discord]
[![Crates.io](https://img.shields.io/crates/v/subtale-cortex?style=flat-square&labelColor=2060d3&color=f0f0f1)][crate]
[![MIT License](https://img.shields.io/badge/license-MIT-f0f0f1?style=flat-square&labelColor=2060d3)][mit]
[![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-f0f0f1?style=flat-square&labelColor=2060d3)][apache]

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
