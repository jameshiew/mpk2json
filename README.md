# mpk2json

Small CLI tool to convert [MessagePack](https://msgpack.org/) files to JSON.

## Usage

Prints JSON directly to stdout for redirection or piping to another tool.

```shell
mpk2json path/to/file.msgpack > path/to/file.json  # save to file
mpk2json path/to/file.msgpack | jq --color-output | less  # quick preview
```
