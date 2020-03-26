# toml-bin
A utility just do one thing: converts [TOML] file into [JSON]

## How to Install
### Build with PKGBUILD (Arch Linux)
Make a new temporary folder and [download PKGBUILD](https://raw.githubusercontent.com/xnuk/toml-bin/master/PKGBUILD) into there, and run `makepkg -si` in your terminal.

```sh
# Make a new temporary folder. For example:
cd $(mktemp -d)

# Download PKGBUILD
curl --get https://raw.githubusercontent.com/xnuk/toml-bin/master/PKGBUILD > PKGBUILD

# Build & Install
makepkg -si
```

### Build from source
After [installing Rust][Rust] and do this in your terminal:

```sh
git clone https://github.com/xnuk/toml-bin
cd toml-bin
cargo build --release

# Usage:
cat /path/to/file.toml | ./target/release/toml

# Feel free to move this anywhere:
cp ./target/release/toml ~/.local/bin/
```


[TOML]: https://github.com/toml-lang/toml
[JSON]: https://www.json.org
[Rust]: https://www.rust-lang.org/
