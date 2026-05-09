# codegen-tests

## Generation tests

Run the DBC fixture generation tests for Rust and C++:

```sh
cargo test -p codegen-tests --lib
```

The Rust test generates `../data/generated.rs`, checks the `data` crate, and runs the generated Rust tests.

The C++ test generates `../data/generated.hpp`, compiles a small C++23 runner that includes it, and runs the generated C++ tests. It uses the platform `c++` compiler by default, or the compiler configured with `CXX`.

Files under `test-files/currently-work/` are expected to generate, compile, and pass their generated tests. Files under the other `test-files/` folders are still tested, but are expected to fail somewhere in the generation/compile/test pipeline.

## Snapshot tests

Run only the Rust and C++ codegen snapshot tests:

```sh
cargo test -p codegen-tests --test snapshots
```

These tests generate Rust and C++ output from `snapshot-tests/all_cases.dbc` in a temporary directory, then compare the generated output with the checked-in `.snap` files.

If a snapshot test fails because the output changed, `insta` writes a `.snap.new` file next to the existing snapshot.

If the generated output changed intentionally, update the snapshots with:

```sh
INSTA_UPDATE=always cargo test -p codegen-tests --test snapshots
```

Then commit the changed files under `snapshot-tests/snapshots/`.
