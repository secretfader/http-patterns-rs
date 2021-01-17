# Application Patterns in Async Rust

![Continuous Integration](https://github.com/nicholaswyoung/patterns-rs/workflows/Continuous%20Integration/badge.svg)
![Security Audit Status](https://github.com/nicholaswyoung/patterns-rs/workflows/Security%20Audit/badge.svg)

This repository contains implementations of several common tasks, often related to developing and scaling HTTP APIs, in async-Rust.

## Examples

[**Warp Users Proxy**](./warp-users-proxy) implements several HTTP endpoints that are common for REST web applications, including user account registration (with configurable input validation) and formatting data received from a remote JSON API.

## License

Copyright 2020 Nicholas Young. Released under the [Mozilla Public License ("MPL"), version 2.0](LICENSE).
