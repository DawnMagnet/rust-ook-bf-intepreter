# Rob-VM: Rust Ook & Brainfuck Virtual Machine

[English](README.md) | [简体中文](README.zh.md)

A fast and efficient interpreter for Brainfuck and Ook programming languages written in Rust.

## Features

- Supports multiple esoteric languages:
  - Brainfuck
  - Ook!
  - Short Ook (compressed Ook syntax)
- Command-line interface with subcommands
- Configurable memory sizes
- Verbose debugging output option
- Custom character mapping for Short Ook
- File or stdin input support

## Installation

```sh
cargo install rob-vm
```

then run binary directly

```sh
$ rob
A cmdline tools to run brainfuck & ook & shortook code

Usage: rob [OPTIONS] <COMMAND>

Commands:
  brainfuck  Run brainfuck code
  ook        Run ook code
  short-ook  Run in short ook mode
  help       Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose
      --max-data-size <MAX_DATA_SIZE>      [default: 50000]
      --max-output-size <MAX_OUTPUT_SIZE>  [default: 50000]
  -h, --help                               Print help
  -V, --version                            Print version

```

if you want to use a statically linked binary, please compile with

```sh
cargo install rob-vm --target x86_64-unknown-linux-musl # for x86_64 linux
cargo install rob-vm --target aarch64-unknown-linux-musl # for aarch64 linux
```

# Usage

you can input from a file(`b` means `brainfuck`, `o` means `ook`, `so` means `short-ook`)

```sh
rob ook a.txt # equivalent to rob o a.txt
rob b b.txt # equivalent to rob brainfuck b.txt
rob so c.txt # equivalent to rob short-ook c.txt
```

or just input a string(Hello World Example)

```sh
rob b ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+."

rob o "Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook.
Ook! Ook. Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook?
Ook! Ook! Ook? Ook! Ook? Ook. Ook. Ook. Ook! Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook. Ook! Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook! Ook. Ook. Ook? Ook. Ook? Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook! Ook.
Ook. Ook? Ook. Ook? Ook. Ook? Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook! Ook? Ook? Ook. Ook. Ook.
Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook. Ook? Ook! Ook! Ook? Ook! Ook? Ook. Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook.
Ook? Ook. Ook? Ook. Ook? Ook. Ook? Ook. Ook! Ook. Ook. Ook. Ook. Ook. Ook. Ook.
Ook! Ook. Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook.
Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook! Ook!
Ook! Ook. Ook. Ook? Ook. Ook? Ook. Ook. Ook! Ook."
```

if you want to specify the memory size, you can use `--max-data-size` and `--max-output-size` options(both default to 50000)

```sh
rob b b.txt --max-data-size 1000 --max-output-size 1000
```
