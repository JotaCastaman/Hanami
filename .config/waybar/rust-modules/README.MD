## 🦀 Waybar Rust Modules

Custom Waybar modules written in Rust, replacing the old shell scripts. A single binary (`waybar-mods`) with one subcommand per module — source in [`.config/waybar/rust-modules`](.config/waybar/rust-modules).

| Command | Module |
|---------|--------|
| `waybar-mods cava` | Audio visualizer (▁▂▃▄▅▆▇█) fed by [cava](https://github.com/karlstav/cava) |
| `waybar-mods mpris [player]` | Now playing (default: `spotify`) with a looping marquee title |
| `waybar-mods language [filter]` | Sway keyboard layout (BR/US), updated by Sway input events — `filter` is part of the keyboard identifier |

All modules run continuously, print only when something changes, and exit together with Waybar, so restarting the bar never leaves orphaned processes behind.

<details>
<summary>📦 Install on a fresh system</summary>

```sh
# Dependencies (Void Linux)
sudo xbps-install -S sway waybar cava playerctl curl base-devel

# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install the binary
cd ~/.config/waybar/rust-modules
cargo build --release
install -Dm755 target/release/waybar-mods ~/.config/waybar/bin/waybar-mods
```

Also needed: Spotify (for the MPRIS module) and the JetBrains Mono Nerd Font (icons).

`target/` and `bin/` are git-ignored, so the build step is required after every fresh clone. The `exec` paths in `config.jsonc` are absolute (`/home/jotacastaman/...`): adjust them if the username changes.

</details>
