# Maintainer: Donovan Glover <https://donovan.is/>
pkgname=sakaya
pkgver=0.1.0
pkgrel=1
pkgdesc="Run native wine applications inside declarative systemd-nspawn containers"
arch=('any')
url="https://github.com/donovanglover/sakaya"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/donovanglover/$pkgname/archive/$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$pkgname-$pkgver"

  cargo build --release --locked
}

package() {
  cd "$pkgname-$pkgver"

  install -Dm755 "target/release/sakaya" "$pkgdir/usr/bin/sakaya"

  install -Dm644 "target/completions/_sakaya" "$pkgdir/usr/share/zsh/site-functions/_sakaya"
  install -Dm644 "target/completions/sakaya.bash" "$pkgdir/usr/share/bash-completion/completions/sakaya"
  install -Dm644 "target/completions/sakaya.fish" "$pkgdir/usr/share/fish/vendor_completions.d/sakaya.fish"
  install -Dm644 "target/man/sakaya.1" "$pkgdir/usr/share/man/man1/sakaya.1"

  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
