# Hack Assembler

This is an assembler made as the final assignment for the [Nand To Tetris Course: Part 1](https://nand2tetris.org/project06). It assumes that the assembly file does not have errors for now.

## Requirements

You need the Rust programming language to develop, build and run this project. You can install its toolchain through [Rustup](https://rustup.rs/)

## Installation

1. Clone this repo:

```
git clone https://github.com/hazemKrimi/hack-assembler
```

2. To run this against hack assembly files run the following command with the path of the `asm` file:
```
cargo run <path>
```
The result `hack` file will be written next to the source file.

## Build

- To build this project run the following command:

```
cargo build
```

- To build for release run the following command:

```
cargo build --release
```

You will find the executable in the `target` directory under `debug` or `release` depending on the command you chose to run from the above.

## Reference

This project is based on the [Nand To Tetris Course: Part 1](https://www.nand2tetris.org/).
