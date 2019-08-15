# Rags

[![Build Status](https://travis-ci.com/mxheller/rags.svg?branch=master)](https://travis-ci.com/mxheller/rags)
[![Coverage Status](https://coveralls.io/repos/github/mxheller/rags/badge.svg?branch=master)](https://coveralls.io/github/mxheller/rags?branch=master)
[![Version info](https://img.shields.io/crates/v/rags.svg)](https://crates.io/crates/rags)

Rags (Rust Alias Generator for Shells) is a tool for analyzing shell history files and suggesting commands to alias.

## Examples

```bash
> rags suggest $HISTFILE
+------+--------------------+------------------------------+
| Uses | Last Used          | Command                      |
+------+--------------------+------------------------------+
| 33   | 2019-05-25 10:20AM | rg --files --hidden          |
+------+--------------------+------------------------------+
| 36   | 2019-05-25 10:20AM | rg --files                   |
+------+--------------------+------------------------------+
| 34   | 2019-01-29 07:14AM | git remote                   |
+------+--------------------+------------------------------+
| 23   | 2019-05-25 10:20AM | rg --files --hidden --follow |
+------+--------------------+------------------------------+
| 22   | 2019-01-29 07:14AM | git remote add               |
+------+--------------------+------------------------------+
```

## Installation

After [installing rust](https://www.rust-lang.org/tools/install), you can install `rags` with:

```
cargo install rags
```

Note the installation path `cargo` outputs at the end, e.g. `/home/max/.cargo/bin/rags`, and make sure that the containing folder (`/home/max/.cargo/bin` in this case) is in your `PATH`.

## Usage

```
USAGE:
    rags <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    suggest    Generates a table of suggested commands to alias
```

### Generating Suggestions

```
USAGE:
    rags suggest <histfile> [n]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <histfile>    Path to history file
    <n>           Number of aliases to suggest [default: 5]
```