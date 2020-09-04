<h2 align="center">Freshfetch</h2>
<p align="center">
<i>A fresh take on Neofetch</i>
<br>
<br>
<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<!--<a href="https://github.com/k4rakara/freshfetch/releases"><img src="https://img.shields.io/github/release/freshfetch/freshfetch.svg"></a>-->
</p>

Freshfetch is an alternative to [Neofetch](https://github.com/dylanaraps/neofetch)
written in Rust with a focus on customization.

## Warning:
Freshfetch is not to a "completed" state yet. Right now, it's in a "beta" of
sorts, with a lot of room for optimization and improvement. If you run into any
problems, be sure to file an issue so that it can be fixed!

## Todo:

 - Optimizations galore
 - A tar.gz package, as well as packages for various distros.
 - Documentation :sweat:
 - Support for images
 - Better portability
 - Add colorization for all distros (69/261 complete)

## Installation

On Arch Linux, you can install the [`freshfetch-git`](https://aur.archlinux.org/packages/freshfetch-git/) package from the AUR. ( You'll also need the [`libcpuid`](https://aur.archlinux.org/packages/freshfetch-git) AUR package. )

For other distros, you can compile from source. Heres what you'll need:
 - make dependencies:
   - `cargo`. I recommend installing this through `rustup`.
 - runtime dependencies:
   - `libcpuid`
   - `luajit`

To compile Freshfetch, just run `cargo build --release -vv`. This will build the executable for your platform. Then, run these commands:
```bash
sudo cp ./target/release/freshfetch /usr/bin/
sudo chmod 755 /usr/bin/freshfetch
```

<p align="center">
<img alt="An example configuration" src="./readme/config-1.png"/>
<img alt="An example configuration" src="./readme/config-2.png"/>
<img alt="An example configuration" src="./readme/config-3.png"/>
</p>

