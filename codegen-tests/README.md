# codegen-tests

## Generation tests

Run the DBC fixture generation tests:

```sh
cargo test -p codegen-tests --lib
```

Files under `test-files/currently-work/` are expected to generate, compile, and pass the generated Rust tests. Files under the other `test-files/` folders are expected to fail somewhere in that pipeline.

By default the output is compact. To show the underlying `data` crate output and expected failure details:

```sh
CODEGEN_TEST_DEBUG=1 cargo test -p codegen-tests --lib
```

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
