pkgname=toml-bin-git
pkgver=0.2.0.r8.059082f
pkgrel=1
pkgdesc='Converts TOML file into JSON'
arch=('x86_64')
url="https://github.com/xnuk/toml-bin.git"
license=('custom:CC0-1.0')

makedepends=('git' 'rust')
provides=("${pkgname%-*}")
conflicts=("${pkgname%-*}")
source=("${pkgname%-*}::git+$url")
sha1sums=('SKIP')

pkgver() {
	cd "${pkgname%-*}"
	printf '%s.r%s.%s' $(awk -F'"' '/^version = /{print $2; exit}' Cargo.toml) $(git rev-list --count HEAD) $(git rev-parse --short HEAD)
}

build() {
	cd "${pkgname%-*}"
	cargo build --release --locked
}

package() {
	cd "${pkgname%-*}"
	install -Dm755 target/release/toml "$pkgdir/usr/bin/toml"
	install -Dm644 LICENSE "$pkgdir/usr/share/licenses/${pkgname%-*}/LICENSE"
}
