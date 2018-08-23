# changelog
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Changelog generator.

- [Documentation][8]
- [Crates.io][2]
- [Releases][9]

## Why?
Keeping track of what changed between versions can be tricky. So changelogs
exist to document those changes for you. But writing changelogs can take a lot
of time, which not everyone has. Which means a lot of projects choose not to
keep a changelog.

So this project exists as a best-effort to generate changelogs for you. It
doesn't require [custom commit styles](https://conventionalcommits.org/) to
work. Nor does it ask you to [manually track
changes](https://keepachangelog.com/en/1.0.0/) in several places.

Instead we automate as much as possible, and present a changelog that's both
easy on the eyes, and easy to use. It's not intended to replace manual
changelogs, but instead provide an option to keep a changelog for projects that
otherwise wouldn't. We hope you find this useful!

## Usage
```txt
changelog 0.1.2
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
