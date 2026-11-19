# Notepad (Rust + Iced)

A fast, lightweight, cross-platform Notepad built with **Rust** and **Iced 0.14**.

## Supported Platforms

- **Linux (Wayland)** — native `wayland` backend + XDG Desktop Portal file dialogs
- **Linux (X11)** — native `x11` backend + GTK/Portal file dialogs
- **Windows 10 / 11** — hardware-accelerated DirectX 12 / Vulkan via `wgpu` + native Win32 dialogs
- **macOS** — hardware-accelerated Metal via `wgpu` + native AppKit dialogs
- **Web Browsers (WebAssembly)** — WebGL2 / WebGPU via `trunk` + browser file picker & download + `localStorage` persistence

## Supported File Formats

Automatic file-type detection, categorized Open/Save dialog filters, browser MIME types (WASM), status-bar language indicator, and streaming syntax highlighting (`iced_highlighter`) for:

- **Text**: `.txt`, `.text`, `.log`
- **Markdown**: `.md`, `.markdown`
- **Configuration**: `.json`, `.xml`, `.yaml`, `.yml`, `.toml`, `.ini`, `.conf`
- **Code**: `.rs`, `.py`, `.js`, `.ts`, `.html`, `.css`, `.c`, `.cpp`, `.java`, `.sh`
- **Data**: `.csv`, `.tsv`, `.sql`
- **Other**: `.bat`, `.ps1`, `.svg`

## Features

- **Clean Modern Design**: Faithful Light & Dark themes (plus System auto-detection), custom frameless tab titlebar with window controls (`─`, `□`, `✕`), rounded active tabs with dirty dot indicator (`•`), and `+` new-tab button.
- **Multi-Tab Editing**: Create, switch, and close tabs (`Ctrl/Cmd+T`, `Ctrl/Cmd+W`).
- **Menu Bar (`File`, `Edit`, `View`) + Settings Gear**: Complete floating Fluent dropdowns and dedicated Settings view (theme, font family, font size, word wrap, syntax highlighting, status bar, formatting toolbar).
- **Lightweight Formatting Toolbar & Formatted View**: Heading (`H1`), List, Bold (`B`), Italic (`I`), Link, and Clear Formatting buttons, plus a one-click **Formatted / Plain text** toggle in the status bar to preview Markdown formatting.
- **Find & Replace Overlay**: Floating Fluent bar (`Ctrl/Cmd+F`, `Ctrl/Cmd+H`) with match counter, Next/Previous navigation, Match Case toggle, Replace, and Replace All.
- **Status Bar**: Live `Ln, Col`, comma-formatted character count, `Formatted` / `Plain text` toggle, detected File Type, Zoom (`10%–500%`), Line Endings (`Windows (CRLF)` / `Unix (LF)`), and `UTF-8`.
- **Performance Optimizations**:
  - Reactive rendering (0% idle CPU/GPU usage).
  - O(1) cached character/line metrics updated only on edits.
  - Streaming syntax highlighting that activates only for non-plain-text formats.
  - Non-blocking async file I/O (`tokio::fs` on desktop, Web File API on WASM).
  - `wgpu` GPU rendering with automatic `tiny-skia` software fallback.
  - Full LTO (`lto = "fat"`), single codegen unit, symbol stripping, and `opt-level = "z"` WASM profile.

## Quick Start

### Prerequisites

Install Rust (1.85+ for Edition 2024):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Desktop (Linux Wayland / Linux X11 / Windows 10/11 / macOS)

```bash
# Run optimized release build
cargo run --release
```

**Linux notes:**
- Both **Wayland** and **X11** are compiled in by default; Iced automatically selects the active display server.
- To force a specific backend on Linux:
  - Wayland: `WINIT_UNIX_BACKEND=wayland cargo run --release`
  - X11: `WINIT_UNIX_BACKEND=x11 cargo run --release`
- On Debian/Ubuntu, install Vulkan/Wayland/X11/GTK dev headers if building from source:
  ```bash
  sudo apt install pkg-config libwayland-dev libxkbcommon-dev libx11-dev libgtk-3-dev
  ```

### Web Browser (WebAssembly)

```bash
# 1. Add the WASM target
rustup target add wasm32-unknown-unknown

# 2. Install Trunk
cargo install trunk

# 3. Serve locally (opens http://127.0.0.1:8080)
trunk serve --release

# Or build static production bundle into ./dist/
trunk build --release
```

## Keyboard Shortcuts

| Action | Shortcut |
| --- | --- |
| New Tab | `Ctrl/Cmd + T` or `Ctrl/Cmd + N` |
| Open File | `Ctrl/Cmd + O` |
| Save / Save As | `Ctrl/Cmd + S` / `Ctrl/Cmd + Shift + S` |
| Close Tab | `Ctrl/Cmd + W` |
| Undo / Redo | `Ctrl/Cmd + Z` / `Ctrl/Cmd + Y` |
| Find / Replace | `Ctrl/Cmd + F` / `Ctrl/Cmd + H` |
| Zoom In / Out / Reset | `Ctrl/Cmd + =` / `Ctrl/Cmd + -` / `Ctrl/Cmd + 0` |
| Insert Time/Date | `F5` |
| Toggle Settings | `Ctrl/Cmd + ,` |
| Close Overlay / Menu | `Esc` |
