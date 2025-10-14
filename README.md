# cargo-xfmt

This project enables the use of unstable Rust formatting options without using nightly Rust

### Installation

```bash
cargo install cargo-xfmt
```

### Usage

The same as `cargo fmt`

```bash
cargo xfmt
cargo xfmt --config ./path/to/config.toml
```

### Format on save

#### VSCode

Install this plugin [https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave](https://marketplace.visualstudio.com/items?itemName=emeraldwalk.RunOnSave)

And add this to `.vscode/settings.json`

```json
{
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": ".rs",
        "cmd": "cargo xfmt --file ${file}"
      }
    ]
  }
}
```

### How it works and why it's relatively safe

While unstable formatting rules do change between versions of Rust, the unstable formatting rules
are built into the stable version of Rust you are using and don't change unless the version of Rust
is updated.

Conventionally you would use

```
cargo +nightly fmt
```

However this will download/use the nightly toolchain which is a moving target. If you use this and
return to a repo after some time - formatting may fail due to a newer version of nightly Rust being installed.

That said, Cargo can be forced to use the current stable version of Rust to run the unstable formatting commands.
This ensures that CI tasks that use unstable formatting _always_ work when run against the same stable toolchain.

This is done by passing rules to `cargo fmt` directly. This cannot be done with a config file.

```
cargo fmt -- --config imports_granularity=Item --config group_imports=StdExternalCrate
```

This CLI tool simply wraps `cargo fmt`. It reads a `rustfmt.toml` and generates the equivalent `cargo fmt` command.
