---
name: notepad-maintainer
description: Comprehensive architecture guide, invariant rules, and maintenance runbook for AI agents maintaining the Notepad codebase.
---

# Notepad AI Maintainer Skill

This skill documents the complete architectural mental model, non-negotiable invariants, and operational workflows for maintaining the **Notepad** repository. Any AI agent modifying, optimizing, or extending this codebase must follow these guidelines.

---

## 1. Architectural Mental Model

Notepad is a high-performance, lightweight Windows 11 Fluent-style text editor built with **Rust** and **Iced 0.14**.

### Directory Structure & Responsibilities

```
src/
├── main.rs          # Application bootstrap, window sizing, settings loader, Iced application runner.
├── app.rs           # Core state machine: NotepadApp struct, Message enum, update() dispatcher, view() tree.
├── document.rs      # Tab model, text_editor::Content buffer, incremental line/character metrics, undo coalescing.
├── theme.rs         # FluentPalette definitions (Light, Dark, AMOLED), Theme::custom palette, widget styling closures.
├── icons.rs         # Embedded vector SVGs and logo rendering. Tinted via Iced Svg styled with currentColor.
├── io.rs            # Fast I/O, zero-copy buffer decoding, line-ending detection (CRLF vs LF), atomic file writes.
├── settings.rs      # UserSettings model, JSON serialization to ~/.config/notepad-iced/settings.json.
└── ui/
    ├── titlebar.rs  # Tab strip, new tab button, window drag region, native caption controls (min/max/close).
    ├── menubar.rs   # Top menu bar (File, Edit, View), formatting toolbar (H1-H6, bold, italic, etc.), settings gear.
    ├── editor.rs    # Core text editing area, formatted preview mode (Markdown inline spans), find/replace highlight.
    ├── statusbar.rs # Bottom status bar (Line/Col, character count, encoding, line endings, zoom level).
    └── overlays.rs  # Popups: Menu dropdowns, Find & Replace floating bar, Settings flyout.
```

---

## 2. Non-Negotiable Invariants

When making changes, every AI agent must strictly preserve the following contracts:

### 2.1 $O(1)$ Typing Latency
- **Never** compute full-buffer string scans, `lines().count()`, or regex evaluations inside the `update(Message::EditorAction)` critical path or during `view()`.
- Incremental line and character counts are maintained inside `document.rs` (`Tab::cached_line_count`, `Tab::cached_char_count`).
- Undo snapshots are coalesced to prevent memory exhaustion and UI stalls.

### 2.2 Pure Monochrome Design (Zero Blue)
- The entire UI is strictly monochrome (white, gray, black) across Light, Dark, and AMOLED modes.
- Never use default Iced blue primary highlights or unstyled controls:
  - All toggler switches must use `theme::fluent_toggler(p)`.
  - All pick lists must use `theme::fluent_pick_list(p)`.
  - All action buttons must use `theme::accent_button(p)` or `theme::ghost_button(p)`.
  - Tab close buttons must use `theme::tab_close_button(p)`.
  - The custom Iced palette (`theme.iced_theme()`) is registered with pure monochrome primary colors.

### 2.3 Resvg / Tiny-Skia SVG Vector Geometry
- In `src/icons.rs`, `resvg` renders embedded SVGs.
- **Never** use stroked zero-height or zero-width `<line>` elements where `y1 == y2` or `x1 == x2` without area (resvg culls them).
- Always use filled `<rect ... fill="currentColor"/>` or multi-vertex angled paths with explicit non-zero bounding boxes.

### 2.4 Frameless Window & Wayland/X11 Compatibility
- Desktop builds run with `decorations: false`.
- The titlebar must always include the `mouse_area` drag region with `Message::DragWindow` and double-click `Message::MaximizeWindow`.
- Keep window caption buttons gated under `#[cfg(not(target_arch = "wasm32"))]`.

---

## 3. Maintenance Runbook

### 3.1 Verification Commands
Before submitting any modification, always execute:

```bash
# 1. Compile & type check
cargo check

# 2. Run unit and integration tests
cargo test

# 3. Format check
cargo fmt --check

# 4. Clippy analysis
cargo clippy --all-targets -- -D warnings

# 5. Build optimized release binary
cargo build --release
```

### 3.2 Adding a New Setting
1. Add the field to `UserSettings` in `src/settings.rs` with serde defaults.
2. Add corresponding `Message` variants in `src/app.rs` and handle them in `update()`.
3. Add the UI control inside `src/ui/overlays.rs` in `settings_flyout()`.
4. Apply `theme::fluent_toggler(p)` for boolean flags or `theme::fluent_pick_list(p)` for options.

### 3.3 Adding a New Icon
1. Define the SVG bytes constant in `src/icons.rs` with `viewBox="0 0 12 12"` or `viewBox="0 0 16 16"`.
2. Use `fill="currentColor"` or `stroke="currentColor"`.
3. Wrap with a public helper function calling `tinted_icon(...)`.

### 3.4 Capturing App-Only Screenshots (Wayland)
To take clean, window-only screenshots without desktop or wallpaper:
```bash
notepad &
PID=$!
sleep 2.5
python3 -c '
import subprocess, re
raw = subprocess.check_output(["lswt", "-j"]).decode()
for block in raw.split("{"):
    if "Notepad" in block:
        m = re.search(r"\"identifier\":\s*([0-9a-fA-F]+)", block)
        if m:
            subprocess.run(["grim", "-T", m.group(1), "assets/screenshots/screenshot.png"], check=True)
'
kill $PID
```
