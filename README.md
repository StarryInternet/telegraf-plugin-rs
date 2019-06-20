# Telegraf Rust Plugin

Allowing you to write telegraf plugins in Rust.

## Adding to a project

Copy `makefile-template.toml` and `lib.go` from this project into the root of
your project directory.

Edit the `makefile-template.toml` script, specifically the location of where
your static library is built. Make sure to change your project to build a static
library with `crate_type = ["staticlib"]` in your `Cargo.toml`!

## How this works

Using some glue Go code a ["go plugin" ](https://golang.org/pkg/plugin/)
is generated to be used in telegraf. Rust talks to the Go using C FFI and is
statically linked into the resulting dynamic library.

A cargo make script is necessary because `go` needs to build the final artifact.
It generates magic in its `.so` that allows other Go code to load it at runtime.
