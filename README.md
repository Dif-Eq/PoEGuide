A tracker UI with optional overlay for the Path of Exile 2 campaign guide written here: https://www.poe2wiki.net/wiki/Guide:Acts_quick_guide
Path of Exile 1 support coming soon.

This entire project was made using Claude AI. I don't know how to write anything in rust. I just like this UI library's features.

For questions or bug reports, open a thread in [Discussions](https://github.com/Dif-Eq/PoEGuide/discussions) or reach out on Discord: **difeq**

## Download

Pre-built Windows binaries are available on the [Releases page](https://github.com/Dif-Eq/PoEGuide/releases). Download `poe2-guide-windows.zip` and extract both files to the same folder. No installation required.

## Structure

```
crates/
  shared/    — shared data (all act steps) and save state logic
  tracker/   — main checklist UI (the full-featured app)
  overlay/   — lightweight always-on-top overlay
```

## Building

Build everything at once (recommended):
```
cargo build --release
```

Binaries will be at:
- `target/release/tracker.exe`
- `target/release/overlay.exe`

## Usage

Run **tracker.exe** as your main checklist — use it to review and check off steps.

The zip includes **overlay.exe** — launch it from the "Open Overlay" button in the tracker UI.

Both apps share the same save file (`%LOCALAPPDATA%\poe2_guide\progress.json`),
so checking off a step in either one is reflected in the other within ~0.5 seconds.

## Known Issues

- **Overlay may not be click-through** — In rare cases the overlay can block mouse input to the game. This has not been consistently reproduced, but if it happens, close the overlay immediately. Avoid positioning the overlay over areas of the screen you frequently click during combat.

## Notes

- The overlay requires **borderless windowed** mode, or any other non-fullscreen mode, in PoE2 to render on top.

## License

This project is licensed under [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/). You are free to share and adapt it for non-commercial purposes, provided you give appropriate credit and distribute any modifications under the same license.

Campaign data sourced from [poe2wiki.net](https://www.poe2wiki.net/wiki/Guide:Acts_quick_guide), used under CC BY-NC 3.0.
