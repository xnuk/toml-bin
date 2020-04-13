# toml-bin
A utility just do two things:
- converts [TOML] file into [JSON]
- or vice versa.

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

To uninstall it, do:

```sh
pacman -Rsc toml-bin-git
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
