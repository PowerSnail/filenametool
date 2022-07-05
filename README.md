# Filename Tool

A CLI program to manipulate paths and file names. In a nutshell, this is a simple wrapper around Rust's `std::path`, exposing the library with a CLI interface. 

| Command     | Arguments    | Description                              |
| ----------- | ------------ | ---------------------------------------- |
| filename    | path         | Get the filename                         |
| is-absolute | path         | Whether the path is an absolute path     |
| parent      | path         | Return the parent of the input           |
| stem        | path         | Get the filename excluding the extension |
| with-suffix | path, suffix | Get the filename with a different suffix |

## Installation

Filename Tool can be installed via `cargo`:

```sh
cargo install filenametool
```

## Build

Simply clone this repository and run `cargo build`. There's no external dependencies.
