# rust2uml

Make UML from a directory of rust files.

## Install Dependencies

The `dot` binary from graphviz package must exist in your path. This is usually
provided by the `graphviz` package in your package manager of choice.

## Toolchain

Note that the `rust-toolchain.toml` specifies an older nightly toolchain. Newer
toolchains will require source changes. Nightly is required to use
`#![feature(rustc_private)]`.

## Build and run example

The library can be executed via the `ml` example:

- `--src_name` arg will name the generated `.dot` and `.svg` files
- `--src_dir` should point to the `src/` directory of the crate that you are
  modeling

```shell
cargo run --example ml -- --src_name my_crate --src_dir /Users/me/crates/my_crate/src
```

The output will be in the `target/doc/<src_name>` directory. One of the files is
an SVG that can be viewed with a web browser.

## Batch Processing Multiple Modules

The `batch` example can process multiple modules in a single command:

```shell
cargo run --example batch -- /path/to/your/project/runtime
```

This will generate UML diagrams for the main project and all configured modules,
outputting them to `target/doc/<module_name>/`.

To automatically copy the generated diagrams to your documentation folder, add
the `--copy` flag:

```shell
cargo run --example batch -- /path/to/your/project/runtime --copy
```

This will generate UML and copy all generated diagrams to
`/path/to/your/project/documentation/runtime_architecture/module_uml/`.

The full list of modules that this script will document can be found in
`./examples/batch.rs`. If new modules are added to the runtime, they also need
to be added here.

## Usage

All command line args can be viewed in `examples/main.rs`.
