# changelog
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Changelog generator.

- [Documentation][8]
- [Crates.io][2]
- [Releases][9]

## Usage
```txt
changelog 0.0.2
Yoshua Wuyts <yoshuawuyts@gmail.com>
Command line parser.

USAGE:
    changelog [FLAGS] [OPTIONS] [path]

FLAGS:
    -h, --help         Prints help information
    -P, --pretty       Enable pretty printing
    -q, --quiet        Suppress all log output
    -V, --version      Prints version information
    -v, --verbosity    Print more log output

OPTIONS:
    -o, --out <file>    Write output to file

ARGS:
    <path>    Project directory [default: .]
```

## Installation
```sh
$ cargo add changelog
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/changelog.svg?style=flat-square
[2]: https://crates.io/crates/changelog
[3]: https://img.shields.io/travis/yoshuawuyts/changelog.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/changelog
[5]: https://img.shields.io/crates/d/changelog.svg?style=flat-square
[6]: https://crates.io/crates/changelog
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/changelog
[9]: https://github.com/yoshuawuyts/changelog/releases
