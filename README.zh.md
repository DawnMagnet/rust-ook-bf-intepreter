# Rob-VM：Rust Ook & Brainfuck 虚拟机

[English](README.md) | [简体中文](README.zh.md)

一个快速高效的 Brainfuck 与 Ook 语言解释器，使用 Rust 编写。

## 特性

- 支持多种小众语言：
  - Brainfuck
  - Ook!
  - Short Ook（压缩版 Ook 语法）
- 带有子命令的命令行接口
- 可配置内存大小
- 可选的详细调试输出
- 支持 Short Ook 自定义字符映射
- 支持文件输入或标准输入

## 安装

```sh
cargo install rob-vm
```

然后直接运行可执行文件

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

如果需要使用静态链接版本，可执行以下命令：

```sh
cargo install rob-vm --target x86_64-unknown-linux-musl # 适用于 x86_64 Linux
cargo install rob-vm --target aarch64-unknown-linux-musl # 适用于 aarch64 Linux
```

# 用法

可以从文件输入（`b` 代表 `brainfuck`，`o` 代表 `ook`，`so` 代表 `short-ook`）：

```sh
rob ook a.txt # 等同于 rob o a.txt
rob b b.txt   # 等同于 rob brainfuck b.txt
rob so c.txt  # 等同于 rob short-ook c.txt
```

或者直接输入字符串（Hello World 示例）：

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

如果需要指定内存大小，可使用 `--max-data-size` 和 `--max-output-size` 选项（均默认为 50000）：

```sh
rob b b.txt --max-data-size 1000 --max-output-size 1000
```
