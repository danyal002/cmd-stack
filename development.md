# Developing CmdStack

## Setting up your Development Environment

## Development

### CLI

#### Building

To build the CLI, run the following command in the root directory:

```
cargo build --bin cmdstack
```

#### Running

To run the CLI, run the following command in the root directory:

```
cargo run --bin cmdstack
```

If you don't want to use `cargo` to run the cli, you can execute the following binary: `./target/debug/cmdstack`

### GUI

#### Building Rust

To build the rust part of the GUI, run the following command in the root directory:

```
cargo build --bin gui
```

#### Running

To run the GUI (display the app with your UI), run the following command in the root directory:

```
cargo tauri dev
```


