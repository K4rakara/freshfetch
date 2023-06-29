<h2 align="center">Freshfetch</h2>

<div align="center">
<i>A fresh take on Neofetch</i>
<br>
<br>

<a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
<a href="https://github.com/k4rakara/freshfetch/releases"><img src="https://img.shields.io/github/v/release/K4rakara/freshfetch"></a> [![Rust](https://github.com/Walker-00/freshfetch/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/Walker-00/freshfetch/actions/workflows/rust.yml)
</div>

Freshfetch is an alternative to [Neofetch](https://github.com/dylanaraps/neofetch)
written in Rust with a focus on customization.

## Warning:
Freshfetch is not to a "completed" state yet. Right now, it's in a "beta" of
sorts, with a lot of room for optimization and improvement. If you run into any
problems, be sure to file an issue so that it can be fixed!

## Todo:

 - Optimizations galore
 - Documentation :sweat:
 - Support for images
 - Add colorization for all distros (69/261 complete)

## Installation

#### Arch Linux

On Arch Linux, you can install one of three AUR packages:

- `freshfetch-git` -- The bleeding-edge version of freshfetch that builds from the master branch.
- `freshfetch-bin` -- The stable version of freshfetch that you just install. No compile required.
- `freshfetch` -- Currently not set up right, will be fixed with the next release. Once set up, It'll build the latest stable version from source.

#### Other distros

With other distributions, you can either install the [latest `tar.gz` build](https://github.com/K4rakara/freshfetch/releases) or build from source.

###### Build from source

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

