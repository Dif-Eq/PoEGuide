A tracker UI with optional overlay for the Path of Exile 2 campaign guide written here: https://www.poe2wiki.net/wiki/Guide:Acts_quick_guide
Path of Exile 1 support coming soon.

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

Or build individually:
```
cargo build --release -p tracker
cargo build --release -p overlay
```

## Usage

Run **tracker.exe** as your main checklist — use it to review and check off steps.

Run **overlay.exe** while playing — it sits on top of your game window and shows
your current step plus the next few upcoming steps.

Both apps share the same save file (`%LOCALAPPDATA%\poe2_guide\progress.json`),
so checking off a step in either one is reflected in the other within ~0.5 seconds.

## Notes

- The overlay requires **borderless windowed** mode in PoE2 to render on top.
