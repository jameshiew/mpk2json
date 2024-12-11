# mpk2json

![Crates.io Version](https://img.shields.io/crates/v/mpk2json)

Small CLI tool to convert [MessagePack](https://msgpack.org/) files to JSON.

## Installation

```
cargo install mpk2json
```

## Usage

Prints JSON directly to stdout for redirection or piping to another tool.

```shell
mpk2json path/to/file.msgpack > path/to/file.json  # save to file
mpk2json path/to/file.msgpack | jq --color-output | less  # quick preview
```
