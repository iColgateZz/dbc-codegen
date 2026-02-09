# dbc-codegen

## Development

### CLI tool

Parse example DBC file into `.txt` file.

```bash
cargo run -- parse
```

Get IR of the DBC file.
Index can be between 0-4

```bash
cargo run -- ir <index>
```

Generate rust file based on the DBC file.
Index can be between 0-4

```bash
cargo run -- gen <index>
```