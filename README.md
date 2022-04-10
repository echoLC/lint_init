# lint-init

A cli for init lint config that wrote by rust.

## Installation

### Install by cargo

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | sh

# Install lint-init
cargo install lint-init

lint-init -h
```

## Usage

### Init config specify dir

```bash
lint-init --template typescript --dir ../react-app
```

### Init config current dir and auto install dependencies

```bash
lint-init --template typescript --dir . --auto-install
```

**Tips: `--auto-install` only work when dir value is `.` **.

## Features

Init lint config for a project.

```console
$ lint-init --version

USAGE:
    lint-init[EXE] [OPTIONS] --template <TEMPLATE>

OPTIONS:
    -t, --template <TEMPLATE>  Template of lint config, the value of template: typescript, reactTs, pureJs, prettier etc.
    -d, --dir              Target dir of generate config, the default value is '.'
    -a, --auto-install     Auto install eslint、eslint-plugin dependencies
    -h, --help             Print help information
    -V, --version          Print version information
```

## TODO

- [x] parse args

- [x] add a variety of templates

- [x] auto create dir and file

- [x] auto run npm/yarn install

- [ ] interact with select template

- [ ] unit test

- [ ] support web assembly
