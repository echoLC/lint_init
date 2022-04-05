# lint-init
A cli for init lint config that wrote by rust.

## feature
Init lint config for a project.

```console
$ lint-init --version

USAGE:
    lint-init[EXE] [OPTIONS] --template <TEMPLATE>

OPTIONS:
    -t, --template <TEMPLATE>  Template of lint config, the value of template: typescript, reactTs, pureJs, prettier etc.
    -d, --dir              Target dir of generate config, the default value is '.'
    -h, --help             Print help information
    -V, --version          Print version information
```    

## TODO
- [x] parse args

- [x] add a variety of templates

- [x] auto create dir and file

- [ ] auto run npm/yarn install

- [ ] interact with select template

- [ ] unit test

- [ ] support web assembly