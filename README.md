# Brainfuck Interpreter

This is a simple Brainfuck interpreter written in Rust.

## Features

- Interprets Brainfuck programs of up to 30000 characters
- Reads Brainfuck programs from files

## Usage

1. Build the interpreter with `cargo build --release`.
2. Run a Brainfuck program with `./target/release/brainfuck_interpreter <file>`, where `<file>` is the path to the Brainfuck program.

## Example

If you have a Brainfuck program in a file called `helloworld.bf`, you can run it with:

```bash
./target/release/brainfuck helloworld.bf
```
