# kaonashi

A testbed for different approaches of finding patterns in a large number of
source files.

## Usage

Just point it to your target directory containing the sources with

```console
$ cargo run --release -- /path/to/src
```

and it will write a file `translations.txt` in your working directory with the
found keys.

To benchmark, do

```console
$ cargo build --release
$ perf stat -r 100 target/release/kaonashi /path/to/src
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
