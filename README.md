# dbc-codegen2

`dbc-codegen2` is a **DBC code generator for CAN bus messages**.

It parses a `.dbc` file, builds an intermediate representation (IR), and generates strongly typed code for working with CAN frames.

Currently supported targets:

- Rust

---

## Installation

Install using Cargo from crates.io:

```bash
cargo install dbc-codegen2
```

After installation the CLI is available as:

```bash
dbc-codegen2
```

## Usage
```bash
dbc-codegen2 <COMMAND> <INPUT> -o <OUTPUT>
```

### Commands:

Commands:

| Command | Description                              |
| ------- | ---------------------------------------- |
| `parse` | Parse a DBC file and print parsed output |
| `ir`    | Show intermediate representation         |
| `gen`   | Generate code from a DBC file            |

You can view help with:

```bash
dbc-codegen2 --help
```
