# Project `tilted`

`tilted` is a toy mathematical interpreter written in Rust. It is a hobbyist
project and is not intended for any serious use.

## Installation

`tilted` consists of the interpreter and the library behind it. If you only want
to try out/test the project, the interpreter is sufficient; otherwise, you can
add the library as a dependency and play around!

### Interpreter

There is no pre-compiled binary available so you will need to compile it
yourself using [`cargo`](https://doc.rust-lang.org/cargo/). Run the following
(make sure `cargo` is installed):

```bash
cargo install tilted
```

### Library

Even though the library is not intended for other usage, it can still be added
as a dependency to your project. Add the following to your `Cargo.toml`:

```toml
[dependencies]
tilted = { version = "=0.3.0", features = [] }
```

`tilted` comes with all features enabled by default, including the `cli` feature
that provides for the executable.

## Usage

The help message can be printed with `tilted --help`:

```text
Usage: tilted [<input>] [-p] [-r]

A non-Turing-complete interpreted programming 'language' that can do maths (only).

Positional Arguments:
  input             user input

Options:
  -p, --ast         print the AST
  -r, --repr        enable interactive (read-eval-print-loop) mode
  --help            display usage information
```
