# pico — Minimal X11 Color Picker  
Fast • Clean • Minimal

<p align="left">
  <img src="logo.png" width="400" alt="pico logo">
</p>

`pico` is a tiny, instant color-picker for X11.  
Click anywhere → get the color in your terminal + automatically copied to clipboard.

Designed for developers, designers, and Linux ricers who demand speed and minimalism.


## ✨ Features

- Pick any color on screen instantly  
- Output formats:
  - HEX (default)
  - `--rgb`
  - `--rgba`
  - `--hsl`
- Symlink variants:
  - `pico-rgb`
  - `pico-rgba`
  - `pico-hsl`
- Auto-copies to clipboard (via `xclip`)
- Terminal color preview block
- Zero UI • Zero Lag • Zero Bloat
- Works on **X11** (Wayland not required)


## Installation

Build from source:

```sh
cargo build --release
```

Install the binary:

```sh
sudo cp target/release/pico /usr/local/bin/
```


## 🔗 Symlinks (Recommended)

Create shortcut binaries that output directly in specific formats:

```sh
sudo ln -s /usr/local/bin/pico /usr/local/bin/pico-rgb
sudo ln -s /usr/local/bin/pico /usr/local/bin/pico-rgba
sudo ln -s /usr/local/bin/pico /usr/local/bin/pico-hsl
```

Now you can run:

```sh
pico-rgb
pico-rgba
pico-hsl
```


## Usage

### Pick a color (default HEX)

```sh
pico
```

### RGB

```sh
pico --rgb
```

### RGBA

```sh
pico --rgba
```

### HSL

```sh
pico --hsl
```

### Help

```sh
pico --h
```

Every output includes a terminal preview:

```
████ #CA3537  
```

---

## 🛠 Dependencies

* `xclip`
* X11 dev libs (`libx11`)

Install on Arch:

```sh
sudo pacman -S xclip libx11
```


## 🤍 License
 
MIT — free to use, free to modify.


## 🌱 Contribute

Open to PRs for:

* Wayland support
* palette extraction
* CMYK
* ASCII color preview
* Sway/i3 scripts

---
# fast • clean • minimal