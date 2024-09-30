## Install rust by executing the following commands  
> Linux

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

The above command installs rustup a tool chain that manages versions of the rust compiler and also installs the latest version of rust on your system

## Check for the version of rust compiler
```bash
$ rustc --version
```

## Opens up the local offline documentation of rust
```bash
$ rustup doc
```

## Check if cargo is installed on your system
```bash
$ cargo --version
```

## Initialising a new rust project using cargo
```bash
cargo new hello_cargo
```

## Initialising a new rust project using cargo with a vcs configured
```bash
cargo new --vcs=git
```
## Building a rust project using cargo
```bash
cargo build
```

## Checking the rust code without producing the output
```bash
cargo check
```

## Building and running the executable with a single command
```bash
cargo run
```

## Builds a documentation for your dependencies and opens up in your browser
```bash
cargo doc --open
```

## Creating a release build 
```bash
cargo build --release
```

## Pointers to remember:
* Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

* Tom’s Obvious, Minimal Language(TOML)
