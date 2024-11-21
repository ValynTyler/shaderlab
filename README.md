# Shader Lab
A place to develop and test computational graphic effects.

## Usage
This project exposes two commands for building and running web content. Use `build` to compile the project to web assembly and then `serve` to view it locally. These commands can be run from anywhere with `nix run <path/to/project>#<command>`, or from a nix development shell with the following:
```bash
$ nix develop <path/to/project>
$ build
...
$ serve
Opening on port `8080`.
```
