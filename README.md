# add-rs

The most advanced code to add a couple of numbers.

## Usage

```
Add.

Usage:
    add <input> <output>
    add (-h | --help)

Options:
  -h --help     Show this screen.
```

Sample input file,

```json
{
    "x": 5,
    "y": 3.5
}
```

Sample output file,

```json
{
    "sum": 8.5,
}
```

## Create a completely static linux binary

Using [Rust's musl support for fully static binaries][rust-musl] you can
create a fully static binary that would run on any linux platform, using
the following commands:

```
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release
```

[rust-musl]: https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
