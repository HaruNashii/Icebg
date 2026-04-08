> [!WARNING]
> This project is in a very early beta stage.
>
> The system is undergoing constant changes and may contain serious bugs, performance issues, or memory leaks.
>
> Use it with caution and expect instability.
>
> I’m currently working on three “ice” projects simultaneously—there’s only so much I can realistically handle at once.


# 🌨️ Icebg

**Icebg** is a lightweight, **Wayland wallpaper setter** written in **Rust**, powered by **iced** and **iced-layer-shell**.

It aims to provide a **minimal, fast, and hackable wallpaper system** designed for wlroots and smithay compositors (Hyprland, Sway, Niri, etc.), with hot-reload support and per-monitor configuration.

> Built for people who want full control instead of heavyweight wallpaper daemons.

---

## ✨ Features

- 🧊 Native Wayland layer-shell wallpaper
- ⚡ Written entirely in Rust
- 🎨 UI powered by `iced`
- 🔄 Hot-reload wallpaper without restarting
- 🖥 Per-monitor targeting
- 🖼 Multiple content fit modes (Contain, Cover, Fill, None, ScaleDown)
- 🧩 Minimal configuration via RON file
- 🪶 Lightweight and compositor-friendly

---

## 🧠 What Icebg Does

Icebg is **not** a traditional wallpaper daemon.

Instead, it acts as a:

- Wayland **layer-shell background surface**
- RON config watcher
- Image renderer

### Core responsibilities

- reads wallpaper path and display config from a RON file
- renders the image as a fullscreen background layer surface
- watches the config file at a configurable interval
- hot-reloads the wallpaper when the config changes
- targets a specific monitor output if configured

Conceptually:
```
RON Config ──read──▶ Icebg Core
                         │
                         ▼
                   iced UI Renderer
                         │
                         ▼
               Wayland Layer Surface (Background)
```

---

## 🖥 Supported Environments

Icebg targets **smithay and wlroots based compositors**, including:

- Hyprland
- Sway
- Niri
- Others layer-shell compatible compositors

X11 is **not supported**.

---

## 📦 Tech Stack

- Rust
- iced (GUI framework)
- iced_layershell
- RON (configuration format)
- Wayland layer-shell protocol

`iced` provides a declarative UI model inspired by Elm architecture.

---

## 🚀 Installation

#### AUR (Recommended):

```paru -S icebg-git```

or

```yay -S icebg-git```

--

#### **Building From Source:**

Requirements for building:
- Rust/Cargo (stable/2024)
- gcc-libs
- A Wayland compositor with layer-shell support

**Build And Install With:**
```bash
git clone https://github.com/HaruNashii/Icebg
cd Icebg
cargo build --release
mkdir -p $HOME/.local/bin
cp -rf target/release/icebg $HOME/.local/bin/
```

**Tip: Run it from your compositor autostart for best results.**

- Example (Hyprland):
```
exec-once = icebg
```

- Example (Sway):
```
exec icebg
```

---

## ⚙️ Configuration

The config file is automatically created on first launch at:
```
~/.config/icebg/config_bg.ron
```

Default config:
```ron
// Tip: Available options for "content_fit" are: "Contain", "Cover", "Fill", "None", "ScaleDown"
BackgroundConfig
(
    display: None,
    wallpaper: "/path/to/your/wallpaper.png",
    content_fit: Fill,
    update_interval: Some(1000),
)
```

### Config Fields

| Field             | Type            | Description                                                  |
|-------------------|-----------------|--------------------------------------------------------------|
| `display`         | `Option<String>`| Target a specific monitor by name. `None` uses the active one|
| `wallpaper`       | `String`        | Absolute path to your wallpaper image                        |
| `content_fit`     | `UserContentFit`| How the image is scaled to fit the screen                    |
| `update_interval` | `Option<u64>`   | Hot-reload interval in milliseconds. `None` disables reload  |

### Content Fit Options

| Option      | Description                                              |
|-------------|----------------------------------------------------------|
| `Contain`   | Scale as big as possible without cropping                |
| `Cover`     | Scale to cover fully, cropping if needed                 |
| `Fill`      | Stretch to fill 100% of the screen                       |
| `None`      | No scaling                                               |
| `ScaleDown` | Scale down only if the image is too large                |

---

## 🧩 Architecture Overview
```
src/
├── main.rs          → application entry point, AppData state
├── ron.rs           → ron configuration handler + hot-reload logic
├── fs.rs            → config file creation and filesystem checks
├── helpers/misc.rs  → image preloading, ContentFit conversion
```

### Key Systems

**1. Layer Shell Integration**
- Creates a fullscreen anchored background surface without a desktop environment.

**2. Config Watcher**
- Polls the RON config file at a user-defined interval.
- Detects changes to wallpaper, content fit, and update interval independently.

**3. Image Renderer**
- Loads the wallpaper via iced's image widget.
- Displays a warning message if the wallpaper path does not exist.

**4. Event Model**
- Icebg follows iced's update/view architecture:
  - Message → Update → State → View

---

## 🎯 Project Goals

**Icebg focuses on:**
- simplicity over feature bloat
- hackability
- learning modern Wayland APIs
- experimenting with iced + layer-shell
- This is intentionally closer to a minimal tool than a full wallpaper daemon.

---

## ⚠️ Current Status

Experimental / Work in Progress.
Expect:
- breaking changes
- incomplete features
- rapid iteration

The project is primarily a learning and experimentation platform.

---

## 🪲 Known Bugs

- Icebg crashing on Gnome.
  - Explanation: Icebg depends on [Layer Shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1#compositor-support) which Gnome hasn't implemented yet!!!
- Changing the `display` field requires a full restart of the app.

---

## 🛠 Roadmap (Planned Ideas)

- Slideshow / timed wallpaper rotation
- Multiple monitor support simultaneously
- Animated wallpaper support (very low priority)
- CLI Tool

---

## 🤝 Contributing

**Contributions are welcome!!!**
**Good areas to help:**

- Wayland handling
- iced widgets
- image loading performance
- compositor testing
- architecture improvements

**Steps:**
```
fork → branch → commit → pull request
```

---

## 📜 License

MIT License.
See [LICENSE](https://github.com/HaruNashii/Icebg/blob/main/LICENSE) for details.

---
