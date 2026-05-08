# codegen-tests

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
