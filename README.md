# add-rs

The most advanced code to add a couple of numbers.

## Downloads

You can get a fully static binary of the `add` code that will run in any
linux computer in the [releases page][releases].

## Usage

```
Add.

Adds two numbers <x> and <y>.

Usage:
    add <x> <y>
    add (-h | --help)

Options:
  -h --help     Show this screen.
```

Sample run,

```bash
$ add 1 3.5
4.5
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
[releases]: https://github.com/odarbelaeze/add-rs/releases