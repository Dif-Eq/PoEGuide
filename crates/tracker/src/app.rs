#![allow(clippy::uninlined_format_args)]

use eframe::egui::{self, Color32, RichText, ScrollArea, Stroke, CornerRadius};
use std::sync::{Arc, Mutex};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "Dif-Eq/PoEGuide";

/// Logical size of the overlay window (must match overlay/src/main.rs with_inner_size).
const OVERLAY_W: f32 = 420.0;
const OVERLAY_H: f32 = 300.0;

use shared::data::{all_acts, Act};
use shared::save::SaveState;

// ─── Monitor enumeration ──────────────────────────────────────────────────────

/// Returns the bounding rect of each monitor in logical pixels (origin = top-left of
/// the primary monitor). Falls back to a single 1920×1080 rect if enumeration fails.
#[cfg(target_os = "windows")]
fn enumerate_monitors(pixels_per_point: f32) -> Vec<egui::Rect> {
    use std::sync::Mutex as StdMutex;
    use windows::Win32::Graphics::Gdi::{EnumDisplayMonitors, HMONITOR, HDC};
    use windows::Win32::Foundation::{BOOL, LPARAM, RECT};

    let monitors: std::sync::Arc<StdMutex<Vec<egui::Rect>>> =
        std::sync::Arc::new(StdMutex::new(Vec::new()));
    let monitors_ptr = std::sync::Arc::clone(&monitors);

    unsafe extern "system" fn monitor_cb(
        _hmon: HMONITOR,
        _hdc: HDC,
        lprect: *mut RECT,
        lparam: LPARAM,
    ) -> BOOL {
        let monitors = &*(lparam.0 as *const StdMutex<Vec<egui::Rect>>);
        if let (Ok(mut list), Some(rc)) = (monitors.lock(), lprect.as_ref()) {
            // Store raw pixel coords; we'll convert after enumeration.
            list.push(egui::Rect::from_min_max(
                egui::pos2(rc.left as f32, rc.top as f32),
                egui::pos2(rc.right as f32, rc.bottom as f32),
            ));
        }
        BOOL(1)
    }

    unsafe {
        let ptr = std::sync::Arc::as_ptr(&monitors_ptr) as isize;
        let _ = EnumDisplayMonitors(HDC::default(), None, Some(monitor_cb), LPARAM(ptr));
    }

    let raw = monitors.lock().unwrap().clone();
    if raw.is_empty() {
        return vec![egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1920.0, 1080.0))];
    }

    // Convert physical pixels → logical pixels and normalize so the virtual
    // origin is at (0, 0) — matching how overlay_x/overlay_y are stored.
    let min_x = raw.iter().map(|r| r.min.x).fold(f32::INFINITY, f32::min);
    let min_y = raw.iter().map(|r| r.min.y).fold(f32::INFINITY, f32::min);
    raw.iter().map(|r| {
        egui::Rect::from_min_max(
            egui::pos2((r.min.x - min_x) / pixels_per_point, (r.min.y - min_y) / pixels_per_point),
            egui::pos2((r.max.x - min_x) / pixels_per_point, (r.max.y - min_y) / pixels_per_point),
        )
    }).collect()
}

#[cfg(not(target_os = "windows"))]
fn enumerate_monitors(_pixels_per_point: f32) -> Vec<egui::Rect> {
    vec![egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1920.0, 1080.0))]
}

// ─── Inline tag parser ────────────────────────────────────────────────────────

#[derive(Clone)]
struct Segment {
    text: String,
    color: Option<Color32>,
}

// Tag colors (defined here so parser can reference them as plain values)
// [b] boss        = crimson
// [i] item        = green
// [s] stat/bonus  = teal
// [q] quest item  = purple
// [z] zone inline = gold
// [o] once/league = orange
const COL_BOSS:  Color32 = Color32::from_rgb(210, 60,  60);
const COL_ITEM:  Color32 = Color32::from_rgb(80,  185, 100);
const COL_STAT:  Color32 = Color32::from_rgb(80,  190, 190);
const COL_QUEST: Color32 = Color32::from_rgb(180, 100, 220);
const COL_ZONE:  Color32 = Color32::from_rgb(210, 185, 95);
const COL_ONCE:  Color32 = Color32::from_rgb(220, 140, 50);
const COL_UNIQUE: Color32 = Color32::from_rgb(175, 96, 37); // PoE2 unique item orange

fn strip_tags(input: &str) -> String {
    let tags = ["[b]","[/b]","[i]","[/i]","[s]","[/s]","[q]","[/q]","[z]","[/z]","[o]","[/o]"];
    let mut out = input.to_string();
    for tag in &tags {
        out = out.replace(tag, "");
    }
    out
}

fn parse_segments(input: &str) -> Vec<Segment> {
    let tags: &[(&str, &str, Color32)] = &[
        ("[b]", "[/b]", COL_BOSS),
        ("[i]", "[/i]", COL_ITEM),
        ("[s]", "[/s]", COL_STAT),
        ("[q]", "[/q]", COL_QUEST),
        ("[z]", "[/z]", COL_ZONE),
        ("[o]", "[/o]", COL_ONCE),
    ];

    let mut segments = Vec::new();
    let mut remaining = input;

    'outer: while !remaining.is_empty() {
        // Find the earliest opening tag
        let mut earliest: Option<(usize, usize, &str, Color32)> = None; // (pos, open_len, close_tag, color)
        for (open, close, color) in tags {
            if let Some(pos) = remaining.find(open) {
                if earliest.is_none() || pos < earliest.unwrap().0 {
                    earliest = Some((pos, open.len(), close, *color));
                }
            }
        }

        if let Some((pos, open_len, close_tag, color)) = earliest {
            // Push plain text before the tag
            if pos > 0 {
                segments.push(Segment { text: remaining[..pos].to_string(), color: None });
            }
            let after_open = &remaining[pos + open_len..];
            if let Some(end) = after_open.find(close_tag) {
                segments.push(Segment { text: after_open[..end].to_string(), color: Some(color) });
                remaining = &after_open[end + close_tag.len()..];
            } else {
                // No closing tag — treat rest as plain
                segments.push(Segment { text: after_open.to_string(), color: None });
                remaining = "";
            }
        } else {
            // No more tags
            segments.push(Segment { text: remaining.to_string(), color: None });
            break 'outer;
        }
    }
    segments
}

// ─── Persistent state (saved to disk) ────────────────────────────────────────

// ─── App ─────────────────────────────────────────────────────────────────────

// Sentinel value for selected_act meaning "show Reference"
const REFERENCE_IDX: usize = usize::MAX;

pub struct GuideApp {
    acts: Vec<Act>,
    selected_act: usize,
    state: SaveState,
    dirty: bool,
    config: shared::config::Config,
    show_settings: bool,
    /// Which hotkey slot is currently being rebound: 0=advance, 1=undo, 2=toggle, None=none
    rebinding: Option<usize>,
    frame_counter: u32,
    /// Running overlay child process, if any
    overlay_process: Option<std::process::Child>,
    /// Update check result: None=checking, Some(None)=up to date, Some(Some(ver))=update available
    update_available: Arc<Mutex<Option<Option<String>>>>,
    /// Whether an update is currently being downloaded
    update_downloading: bool,
}

impl GuideApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let update_available = Arc::new(Mutex::new(None));
        let update_check = Arc::clone(&update_available);
        std::thread::spawn(move || {
            let result = check_for_update();
            if let Ok(mut guard) = update_check.lock() {
                *guard = Some(result);
            }
        });
        Self {
            acts: all_acts(),
            selected_act: 0,
            state: SaveState::load(),
            dirty: false,
            config: shared::config::Config::load(),
            show_settings: false,
            rebinding: None,
            frame_counter: 0,
            overlay_process: None,
            update_available,
            update_downloading: false,
        }
    }

    fn overlay_running(&mut self) -> bool {
        match &mut self.overlay_process {
            None => false,
            Some(child) => {
                // try_wait returns Ok(None) if still running
                if let Ok(None) = child.try_wait() { true } else {
                    self.overlay_process = None;
                    false
                }
            }
        }
    }

    fn launch_overlay(&mut self) {
        // Look for overlay.exe next to the current executable
        let overlay_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("overlay.exe")));

        if let Some(path) = overlay_path {
            if path.exists() {
                match std::process::Command::new(&path).spawn() {
                    Ok(child) => { self.overlay_process = Some(child); }
                    Err(e) => { eprintln!("Failed to launch overlay: {e}"); }
                }
            } else {
                eprintln!("overlay.exe not found at {}", path.display());
            }
        }
    }

    fn close_overlay(&mut self) {
        if let Some(mut child) = self.overlay_process.take() {
            let _ = child.kill();
        }
    }

    fn do_update(&mut self, new_version: &str) {
        self.update_downloading = true;
        let version = new_version.to_string();
        std::thread::spawn(move || {
            let _ = download_and_apply_update(&version);
        });
    }

    fn ref_section(ui: &mut egui::Ui, title: &str, tip: &str) {
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(RichText::new(title).color(ACCENT_GOLD).size(15.0).strong());
        });
        if !tip.is_empty() {
            ui.horizontal(|ui| {
                ui.add_space(12.0);
                ui.label(RichText::new(tip).color(TEXT_DIM).size(12.0));
            });
        }
        ui.add_space(4.0);
    }

}

// ─── Update checking ─────────────────────────────────────────────────────────

fn check_for_update() -> Option<String> {
    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");
    let response = ureq::get(&url)
        .set("User-Agent", "poe2-guide-tracker")
        .call().ok()?;
    let json: serde_json::Value = response.into_json().ok()?;
    let tag = json["tag_name"].as_str()?;
    let remote = tag.trim_start_matches('v');
    let local = semver::Version::parse(VERSION).ok()?;
    let latest = semver::Version::parse(remote).ok()?;
    if latest > local { Some(remote.to_string()) } else { None }
}

fn download_and_apply_update(version: &str) -> Option<()> {
    let url = format!(
        "https://github.com/{GITHUB_REPO}/releases/download/v{version}/tracker.exe"
    );
    let response = ureq::get(&url)
        .set("User-Agent", "poe2-guide-tracker")
        .call().ok()?;

    let current_exe = std::env::current_exe().ok()?;
    let dir = current_exe.parent()?;
    let new_path = dir.join("tracker_new.exe");
    let script_path = dir.join("update.bat");

    // Write new binary
    let mut bytes: Vec<u8> = Vec::new();
    std::io::copy(&mut response.into_reader(), &mut bytes).ok()?;
    std::fs::write(&new_path, &bytes).ok()?;

    // Write batch script that waits for this process to exit, swaps, relaunches
    let current_name = current_exe.file_name()?.to_string_lossy();
    let script = format!(
        "@echo off\r\n\
         :wait\r\n\
         tasklist /fi \"imagename eq {current_name}\" 2>nul | find /i \"{current_name}\" >nul\r\n\
         if not errorlevel 1 (timeout /t 1 /nobreak >nul & goto wait)\r\n\
         move /y \"{new}\" \"{cur}\"\r\n\
         start \"\" \"{cur}\"\r\n\
         del \"%~f0\"\r\n",
        current_name = current_name,
        new = new_path.display(),
        cur = current_exe.display(),
    );
    std::fs::write(&script_path, script).ok()?;

    // Launch the script and exit
    std::process::Command::new("cmd")
        .args(["/c", &script_path.to_string_lossy()])
        .spawn().ok()?;
    std::process::exit(0);
}

// ─── Colour palette (PoE2 logo-inspired) ─────────────────────────────────────
// Background: deep teal-charcoal, like the logo's dark atmosphere
// Accents: cold bright gold (logo lettering) + blood crimson (logo blades/border)
// Panels: aged steel-grey tones

const BG_DARK: Color32    = Color32::from_rgb(10, 13, 16);   // near-black teal
const BG_PANEL: Color32   = Color32::from_rgb(18, 22, 28);   // dark steel
const BG_ZONE: Color32    = Color32::from_rgb(24, 30, 38);   // slightly lighter steel
const ACCENT_GOLD: Color32     = Color32::from_rgb(210, 185, 95);  // cold logo gold
const ACCENT_GOLD_DIM: Color32 = Color32::from_rgb(120, 105, 50); // muted gold
const CRIMSON: Color32         = Color32::from_rgb(160, 30, 30);  // blood red accent
const CRIMSON_BRIGHT: Color32  = Color32::from_rgb(200, 50, 40);  // bright crimson highlight
const TEXT_MAIN: Color32  = Color32::from_rgb(210, 215, 220);  // cool near-white
const TEXT_DIM: Color32   = Color32::from_rgb(120, 130, 140);  // muted steel grey
const TEXT_DONE: Color32  = Color32::from_rgb(65, 72, 80);    // very dim, done items
const PROGRESS_BG: Color32 = Color32::from_rgb(30, 20, 20);  // dark crimson trough
const PROGRESS_FG: Color32 = Color32::from_rgb(165, 35, 35); // crimson fill
const SIDEBAR_SELECTED: Color32 = Color32::from_rgb(35, 20, 20); // crimson tint selected
const WARN_COLOR: Color32       = Color32::from_rgb(210, 130, 40);
// Checkbox colours
const CB_BG: Color32      = Color32::from_rgb(18, 22, 28);   // same as panel — interior
const CB_BORDER: Color32  = Color32::from_rgb(130, 110, 55); // gold border, always visible
const CB_CHECKED_BG: Color32 = Color32::from_rgb(110, 20, 20); // crimson when ticked
const CB_CHECK_MARK: Color32 = Color32::from_rgb(210, 185, 95); // gold checkmark

impl eframe::App for GuideApp {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-save when dirty, otherwise poll for external changes (e.g. overlay advancing steps)
        if self.dirty {
            self.state.save();
            self.dirty = false;
        } else {
            self.frame_counter += 1;
            if self.frame_counter >= 6 {
                self.frame_counter = 0;
                self.state = SaveState::load();
            }
        }

        // Global style
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = BG_DARK;
        style.visuals.panel_fill = BG_PANEL;
        style.visuals.override_text_color = Some(TEXT_MAIN);
        style.visuals.widgets.noninteractive.bg_fill = BG_DARK;
        style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, CB_BORDER);
        style.visuals.widgets.inactive.bg_fill = CB_BG;
        style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, CB_BORDER);
        style.visuals.widgets.hovered.bg_fill = BG_ZONE;
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.5, ACCENT_GOLD);
        style.visuals.widgets.active.bg_fill = CB_CHECKED_BG;
        style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, ACCENT_GOLD);
        style.visuals.selection.bg_fill = CRIMSON;
        style.visuals.selection.stroke = Stroke::new(1.0, CB_CHECK_MARK);
        style.spacing.item_spacing = egui::vec2(0.0, 5.0);
        ctx.set_style(style);

        // Keep repainting until update check resolves
        if matches!(self.update_available.lock().ok().as_deref(), Some(None)) {
            ctx.request_repaint_after(std::time::Duration::from_millis(500));
        }

        // ── Sidebar ──────────────────────────────────────────────────────────
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(170.0)
            .frame(egui::Frame::new().fill(BG_PANEL).inner_margin(egui::Margin::symmetric(0, 0)))
            .show(ctx, |ui| {
                ui.add_space(12.0);

                // Title
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("PoE2 Guide")
                            .color(ACCENT_GOLD)
                            .size(17.0)
                            .strong(),
                    );
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.label(RichText::new("Campaign Checklist").color(TEXT_DIM).size(11.0));
                });

                ui.add_space(10.0);
                ui.add(egui::Separator::default().spacing(1.0));
                ui.add_space(8.0);

                // Reference — same style as act buttons, sits first in the list
                {
                    let is_selected = self.selected_act == REFERENCE_IDX;
                    let bg = if is_selected { SIDEBAR_SELECTED } else { BG_PANEL };
                    let border = if is_selected { CRIMSON } else { Color32::TRANSPARENT };
                    let response = egui::Frame::new()
                        .fill(bg)
                        .stroke(Stroke::new(1.0, border))
                        .inner_margin(egui::Margin { left: 12, right: 8, top: 7, bottom: 7 })
                        .corner_radius(CornerRadius::same(4))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            let name_color = if is_selected { ACCENT_GOLD } else { TEXT_MAIN };
                            ui.label(RichText::new("Reference").color(name_color).size(13.0).strong());
                            ui.label(RichText::new("Gear, weapons, tips").color(TEXT_DIM).size(10.0));
                        })
                        .response
                        .interact(egui::Sense::click());
                    if response.clicked() {
                        self.selected_act = REFERENCE_IDX;
                        self.show_settings = false;
                        self.rebinding = None;
                    }
                    ui.add_space(2.0);
                }

                // Act buttons
                for (i, act) in self.acts.iter().enumerate() {
                    let (done, total) = self.state.act_progress(i, act);
                    let is_selected = self.selected_act == i;
                    #[allow(clippy::cast_precision_loss)]
                    let pct = if total > 0 { done as f32 / total as f32 } else { 0.0 };

                    let bg = if is_selected { SIDEBAR_SELECTED } else { BG_PANEL };
                    let border = if is_selected { CRIMSON } else { Color32::TRANSPARENT };

                    let response = egui::Frame::new()
                        .fill(bg)
                        .stroke(Stroke::new(1.0, border))
                        .inner_margin(egui::Margin { left: 12, right: 8, top: 7, bottom: 7 })
                        .corner_radius(CornerRadius::same(4))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());

                            // Act name
                            let name_color = if is_selected { ACCENT_GOLD } else { TEXT_MAIN };
                            ui.label(RichText::new(act.name).color(name_color).size(13.0).strong());

                            // Progress text
                            ui.label(
                                RichText::new(format!("{done}/{total} steps"))
                                    .color(TEXT_DIM)
                                    .size(10.0),
                            );

                            // Mini progress bar
                            let bar_rect = {
                                let r = ui.allocate_space(egui::vec2(ui.available_width(), 4.0));
                                r.1
                            };
                            ui.painter().rect_filled(bar_rect, 2.0, PROGRESS_BG);
                            if pct > 0.0 {
                                let mut filled = bar_rect;
                                filled.max.x = bar_rect.min.x + bar_rect.width() * pct;
                                ui.painter().rect_filled(filled, 2.0, CRIMSON_BRIGHT);
                            }
                        })
                        .response
                        .interact(egui::Sense::click());

                    if response.clicked() {
                        self.selected_act = i;
                        self.show_settings = false;
                        self.rebinding = None;
                    }
                    if response.hovered() && !is_selected {
                        ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    ui.add_space(2.0);
                }

                // Bottom: reset + settings buttons
                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Reset Act").color(TEXT_DIM).size(11.0),
                                )
                                .fill(BG_PANEL)
                                .stroke(Stroke::new(1.0, CRIMSON)),
                            )
                            .on_hover_text("Clear all checkboxes for this act")
                            .clicked()
                        {
                            let act = &self.acts[self.selected_act];
                            self.state.reset_act(self.selected_act, act);
                            self.dirty = true;
                        }
                        ui.add_space(4.0);
                        let settings_label = if self.show_settings { "< Back" } else { "Settings" };
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(settings_label).color(TEXT_DIM).size(11.0),
                                )
                                .fill(BG_PANEL)
                                .stroke(Stroke::new(1.0, ACCENT_GOLD_DIM)),
                            )
                            .clicked()
                        {
                            self.show_settings = !self.show_settings;
                            self.rebinding = None;
                            // If coming back from settings and on reference, stay on reference.
                            // If on reference and opening settings, that's fine — settings shows over it.
                            if self.show_settings && self.selected_act == REFERENCE_IDX {
                                self.selected_act = 0;
                            }
                        }
                    });
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        let overlay_running = self.overlay_running();
                        let (overlay_label, overlay_color, overlay_stroke) = if overlay_running {
                            ("Close Overlay", TEXT_DIM, Stroke::new(1.0, CRIMSON))
                        } else {
                            ("Open Overlay", TEXT_DIM, Stroke::new(1.0, ACCENT_GOLD_DIM))
                        };
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new(overlay_label).color(overlay_color).size(11.0),
                                )
                                .fill(BG_PANEL)
                                .stroke(overlay_stroke),
                            )
                            .clicked()
                        {
                            if overlay_running {
                                self.close_overlay();
                            } else {
                                self.launch_overlay();
                            }
                        }
                    });
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        ui.label(RichText::new("poe2wiki.net").color(TEXT_DIM).size(9.0).italics());
                    });

                    // Update badge
                    let update_state = self.update_available.lock()
                        .ok()
                        .and_then(|g| g.clone());
                    if let Some(Some(ref new_ver)) = update_state {
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.add_space(12.0);
                            let label = if self.update_downloading {
                                "Downloading..."
                            } else {
                                "⬆ Update available"
                            };
                            if ui.add(
                                egui::Button::new(RichText::new(label).color(Color32::BLACK).size(11.0))
                                    .fill(ACCENT_GOLD)
                                    .stroke(Stroke::new(0.0, Color32::TRANSPARENT))
                            ).clicked() && !self.update_downloading {
                                let ver = new_ver.clone();
                                self.do_update(&ver);
                            }
                        });
                    }
                    ui.add_space(4.0);
                });
            });

        // ── Main content ─────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(BG_DARK).inner_margin(egui::Margin::symmetric(0, 0)))
            .show(ctx, |ui| {

                if self.selected_act == REFERENCE_IDX {
                    // ── Reference ─────────────────────────────────────────
                    egui::Frame::new()
                        .fill(BG_PANEL)
                        .inner_margin(egui::Margin { left: 20, right: 20, top: 14, bottom: 14 })
                        .show(ui, |ui| {
                            ui.label(RichText::new("Reference").color(ACCENT_GOLD).size(20.0).strong());
                            ui.add_space(2.0);
                            ui.label(RichText::new("Levelling gear, weapon mods, and terminology.")
                                .color(TEXT_DIM).size(11.0));
                        });

                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .drag_to_scroll(false)
                        .show(ui, |ui| {
                            ui.add_space(8.0);

                            // ── Unique Gear ───────────────────────────────
                            Self::ref_section(ui, "Unique Items", "Prioritize boots with the highest movement speed you can equip.");
                            egui::Frame::new()
                                .fill(BG_ZONE)
                                .corner_radius(CornerRadius::same(4))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                                .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 8 })
                                .inner_margin(egui::Margin { left: 8, right: 8, top: 0, bottom: 0 })
                                .show(ui, |ui| {
                                    egui::Grid::new("gear_table")
                                        .num_columns(3)
                                        .min_col_width(120.0)
                                        .spacing(egui::vec2(16.0, 4.0))
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("Slot").color(ACCENT_GOLD).size(12.0).strong());
                                            ui.label(RichText::new("Item").color(ACCENT_GOLD).size(12.0).strong());
                                            ui.label(RichText::new("Benefit").color(ACCENT_GOLD).size(12.0).strong());
                                            ui.end_row();
                                            for row in shared::reference::unique_gear() {
                                                ui.label(RichText::new(row.slot).color(TEXT_DIM).size(12.0));
                                                ui.label(RichText::new(row.item).color(COL_UNIQUE).size(12.0));
                                                ui.label(RichText::new(row.benefit).color(TEXT_MAIN).size(12.0));
                                                ui.end_row();
                                            }
                                        });
                                });

                            // ── Weapons ───────────────────────────────────
                            Self::ref_section(ui, "Weapons", "Buy a rare weapon early on, and upgrade every ~10 levels.");
                            egui::Frame::new()
                                .fill(BG_ZONE)
                                .corner_radius(CornerRadius::same(4))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                                .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 8 })
                                .inner_margin(egui::Margin { left: 8, right: 8, top: 0, bottom: 0 })
                                .show(ui, |ui| {
                                    egui::Grid::new("weapon_table")
                                        .num_columns(3)
                                        .min_col_width(140.0)
                                        .spacing(egui::vec2(16.0, 4.0))
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label(RichText::new("Weapon Type").color(ACCENT_GOLD).size(12.0).strong());
                                            ui.label(RichText::new("Key Mod").color(ACCENT_GOLD).size(12.0).strong());
                                            ui.label(RichText::new("Benefit").color(ACCENT_GOLD).size(12.0).strong());
                                            ui.end_row();
                                            for row in shared::reference::weapons() {
                                                ui.label(RichText::new(row.weapon_type).color(TEXT_DIM).size(12.0));
                                                ui.vertical(|ui| {
                                                    for line in row.key_mod.lines() {
                                                        ui.label(RichText::new(line).color(COL_STAT).size(12.0));
                                                    }
                                                });
                                                ui.label(RichText::new(row.benefit).color(TEXT_MAIN).size(12.0));
                                                ui.end_row();
                                            }
                                        });
                                });
                            // ── Terminology ───────────────────────────────
                            Self::ref_section(ui, "Terminology", "");
                            egui::Frame::new()
                                .fill(BG_ZONE)
                                .corner_radius(CornerRadius::same(4))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                                .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 8 })
                                .inner_margin(egui::Margin { left: 16, right: 16, top: 8, bottom: 8 })
                                .show(ui, |ui| {
                                    for entry in shared::reference::terminology() {
                                        ui.horizontal(|ui| {
                                            ui.label(RichText::new(entry.term).color(ACCENT_GOLD).size(13.0).strong());
                                            ui.label(RichText::new(" — ").color(TEXT_DIM).size(13.0));
                                            ui.label(RichText::new(entry.description).color(TEXT_MAIN).size(13.0));
                                        });
                                    }
                                });

                            ui.add_space(20.0);
                        });

                } else if self.show_settings {
                    // ── Settings / Hotkey config ──────────────────────────
                    egui::Frame::new()
                        .fill(BG_PANEL)
                        .inner_margin(egui::Margin { left: 20, right: 20, top: 14, bottom: 14 })
                        .show(ui, |ui| {
                            ui.label(RichText::new("Overlay Hotkeys").color(ACCENT_GOLD).size(20.0).strong());
                            ui.add_space(4.0);
                            ui.label(RichText::new("Click a binding then press your desired key combo (Ctrl/Shift/Alt + key).")
                                .color(TEXT_DIM).size(11.0));
                            ui.label(RichText::new("Changes are picked up by the overlay automatically within 0.5 seconds.")
                                .color(TEXT_DIM).size(11.0));
                        });

                    ui.add_space(12.0);

                    // Capture key input when rebinding
                    if self.rebinding.is_some() {
                        ctx.input(|i| {
                            let ctrl  = i.modifiers.ctrl;
                            let shift = i.modifiers.shift;
                            let alt   = i.modifiers.alt;

                            // Look for any non-modifier key press
                            for key in [
                                egui::Key::F1,  egui::Key::F2,  egui::Key::F3,  egui::Key::F4,
                                egui::Key::F5,  egui::Key::F6,  egui::Key::F7,  egui::Key::F8,
                                egui::Key::F9,  egui::Key::F10, egui::Key::F11, egui::Key::F12,
                                egui::Key::A, egui::Key::B, egui::Key::C, egui::Key::D,
                                egui::Key::E, egui::Key::F, egui::Key::G, egui::Key::H,
                                egui::Key::I, egui::Key::J, egui::Key::K, egui::Key::L,
                                egui::Key::M, egui::Key::N, egui::Key::O, egui::Key::P,
                                egui::Key::Q, egui::Key::R, egui::Key::S, egui::Key::T,
                                egui::Key::U, egui::Key::V, egui::Key::W, egui::Key::X,
                                egui::Key::Y, egui::Key::Z,
                            ] {
                                if i.key_pressed(key) {
                                    let key_str = format!("{key:?}");
                                    let new_hk = shared::config::Hotkey { ctrl, shift, alt, key: key_str };
                                    match self.rebinding.unwrap() {
                                        0 => self.config.hotkey_advance = new_hk,
                                        1 => self.config.hotkey_undo    = new_hk,
                                        2 => self.config.hotkey_toggle  = new_hk,
                                        _ => {}
                                    }
                                    self.config.save();
                                    self.rebinding = None;
                                }
                            }
                            // Escape cancels
                            if i.key_pressed(egui::Key::Escape) {
                                self.rebinding = None;
                            }
                        });
                    }

                    let bindings = [
                        (0usize, "Advance (next step)",   self.config.hotkey_advance.display()),
                        (1,      "Undo (previous step)",  self.config.hotkey_undo.display()),
                        (2,      "Toggle visibility",     self.config.hotkey_toggle.display()),
                    ];

                    for (idx, label, binding) in &bindings {
                        egui::Frame::new()
                            .fill(BG_ZONE)
                            .corner_radius(CornerRadius::same(4))
                            .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                            .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 4 })
                            .inner_margin(egui::Margin { left: 16, right: 16, top: 10, bottom: 10 })
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(*label).color(TEXT_MAIN).size(13.0));
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let is_rebinding = self.rebinding == Some(*idx);
                                        let btn_text = if is_rebinding {
                                            "Press key...".to_string()
                                        } else {
                                            binding.clone()
                                        };
                                        let btn_color = if is_rebinding { ACCENT_GOLD } else { TEXT_MAIN };
                                        let btn_stroke = if is_rebinding {
                                            Stroke::new(1.5, ACCENT_GOLD)
                                        } else {
                                            Stroke::new(1.0, ACCENT_GOLD_DIM)
                                        };
                                        if ui.add(
                                            egui::Button::new(RichText::new(btn_text).color(btn_color).size(12.0))
                                                .fill(BG_PANEL)
                                                .stroke(btn_stroke)
                                                .min_size(egui::vec2(120.0, 0.0)),
                                        ).clicked() {
                                            self.rebinding = if is_rebinding { None } else { Some(*idx) };
                                        }
                                    });
                                });
                            });
                    }

                    // ── Opacity slider ────────────────────────────────────
                    egui::Frame::new()
                        .fill(BG_ZONE)
                        .corner_radius(CornerRadius::same(4))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                        .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 4 })
                        .inner_margin(egui::Margin { left: 16, right: 16, top: 10, bottom: 10 })
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.set_max_width(ui.available_width());
                            ui.label(RichText::new("Overlay Opacity").color(TEXT_MAIN).size(13.0));
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                let mut opacity = self.config.opacity;
                                ui.spacing_mut().slider_width = ui.available_width() - 50.0;
                                let resp = ui.add(
                                    egui::Slider::new(&mut opacity, 0.1..=1.0)
                                        .show_value(false)
                                );
                                if resp.changed() {
                                    self.config.opacity = opacity;
                                }
                                if resp.drag_stopped() {
                                    self.config.save();
                                }
                                ui.add_space(6.0);
                                ui.label(RichText::new(format!("{:.0}%", self.config.opacity * 100.0))
                                    .color(TEXT_DIM).size(12.0));
                            });
                        });

                    // ── Position picker ───────────────────────────────────
                    egui::Frame::new()
                        .fill(BG_ZONE)
                        .corner_radius(CornerRadius::same(4))
                        .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                        .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 4 })
                        .inner_margin(egui::Margin { left: 16, right: 16, top: 10, bottom: 10 })
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.set_max_width(ui.available_width());
                            ui.label(RichText::new("Overlay Position").color(TEXT_MAIN).size(13.0));
                            ui.add_space(2.0);
                            ui.label(RichText::new("Drag the box to set where the overlay appears on screen.")
                                .color(TEXT_DIM).size(11.0));
                            ui.add_space(8.0);

                            // Enumerate all monitors in logical pixels.
                            let monitors = enumerate_monitors(ctx.pixels_per_point());

                            // Virtual bounding box across all monitors.
                            let virt_min_x = monitors.iter().map(|r| r.min.x).fold(f32::INFINITY, f32::min);
                            let virt_min_y = monitors.iter().map(|r| r.min.y).fold(f32::INFINITY, f32::min);
                            let virt_max_x = monitors.iter().map(|r| r.max.x).fold(f32::NEG_INFINITY, f32::max);
                            let virt_max_y = monitors.iter().map(|r| r.max.y).fold(f32::NEG_INFINITY, f32::max);
                            let virt_w = virt_max_x - virt_min_x;
                            let virt_h = virt_max_y - virt_min_y;

                            // Preview area — maintain virtual desktop aspect ratio.
                            let preview_w = 320.0_f32;
                            let preview_h = preview_w * (virt_h / virt_w);
                            let scale = preview_w / virt_w;

                            let (preview_rect, _) = ui.allocate_exact_size(
                                egui::vec2(preview_w, preview_h),
                                egui::Sense::hover(),
                            );

                            // Draw each monitor as a dark rect with a border.
                            for (i, mon) in monitors.iter().enumerate() {
                                let mon_rect = egui::Rect::from_min_max(
                                    egui::pos2(
                                        preview_rect.min.x + (mon.min.x - virt_min_x) * scale,
                                        preview_rect.min.y + (mon.min.y - virt_min_y) * scale,
                                    ),
                                    egui::pos2(
                                        preview_rect.min.x + (mon.max.x - virt_min_x) * scale,
                                        preview_rect.min.y + (mon.max.y - virt_min_y) * scale,
                                    ),
                                );
                                ui.painter().rect_filled(mon_rect, 4.0, Color32::from_rgb(8, 10, 14));
                                ui.painter().rect_stroke(mon_rect, 4.0,
                                    Stroke::new(1.0, Color32::from_rgb(60, 70, 85)), egui::StrokeKind::Outside);
                                // Label each monitor by number.
                                ui.painter().text(
                                    mon_rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    format!("{}", i + 1),
                                    egui::FontId::proportional(10.0),
                                    Color32::from_rgb(50, 60, 75),
                                );
                            }

                            // Overlay box.
                            let box_w = OVERLAY_W * scale;
                            let box_h = OVERLAY_H * scale;
                            let bx = preview_rect.min.x + (self.config.overlay_x - virt_min_x) * scale;
                            let by = preview_rect.min.y + (self.config.overlay_y - virt_min_y) * scale;
                            let box_rect = egui::Rect::from_min_size(
                                egui::pos2(bx, by),
                                egui::vec2(box_w, box_h),
                            );

                            let box_id = ui.id().with("overlay_pos_box");
                            let box_resp = ui.interact(box_rect, box_id, egui::Sense::drag());

                            let box_color = if box_resp.hovered() || box_resp.dragged() {
                                ACCENT_GOLD
                            } else {
                                CRIMSON
                            };
                            ui.painter().rect_filled(box_rect, 2.0,
                                Color32::from_rgba_unmultiplied(160, 30, 30, 120));
                            ui.painter().rect_stroke(box_rect, 2.0,
                                Stroke::new(1.5, box_color), egui::StrokeKind::Outside);
                            ui.painter().text(
                                box_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "overlay",
                                egui::FontId::proportional(8.0),
                                TEXT_DIM,
                            );

                            // Show current pixel position.
                            ui.add_space(4.0);
                            ui.label(RichText::new(
                                format!("x: {:.0}  y: {:.0}", self.config.overlay_x, self.config.overlay_y))
                                .color(TEXT_DONE).size(10.0));

                            if box_resp.dragged() {
                                let delta = box_resp.drag_delta();
                                let new_x = (self.config.overlay_x + delta.x / scale)
                                    .clamp(virt_min_x, virt_max_x - OVERLAY_W);
                                let new_y = (self.config.overlay_y + delta.y / scale)
                                    .clamp(virt_min_y, virt_max_y - OVERLAY_H);
                                self.config.overlay_x = new_x;
                                self.config.overlay_y = new_y;
                            }
                            if box_resp.drag_stopped() {
                                self.config.save();
                            }
                        });

                    // Version
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(12.0);
                        ui.label(RichText::new(format!("v{VERSION}")).color(TEXT_DONE).size(11.0));
                    });
                    ui.add_space(8.0);

                } else {
                let act_idx = self.selected_act.min(self.acts.len().saturating_sub(1));
                let act = &self.acts[act_idx];
                let (done, total) = self.state.act_progress(act_idx, act);
                #[allow(clippy::cast_precision_loss)]
                let pct = if total > 0 { done as f32 / total as f32 } else { 0.0 };

                // Header
                egui::Frame::new()
                    .fill(BG_PANEL)
                    .inner_margin(egui::Margin { left: 20, right: 20, top: 14, bottom: 14 })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Title on the left
                            ui.vertical(|ui| {
                                ui.label(
                                    RichText::new(act.name)
                                        .color(ACCENT_GOLD)
                                        .size(22.0)
                                        .strong(),
                                );
                                ui.label(
                                    RichText::new(act.subtitle)
                                        .color(TEXT_DIM)
                                        .size(12.0),
                                );
                            });

                            // Counter pinned to the right
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.add_space(4.0);
                                // Allocate a fixed-width column for the counter
                                ui.allocate_ui_with_layout(
                                    egui::vec2(90.0, 40.0),
                                    egui::Layout::top_down(egui::Align::Max),
                                    |ui| {
                                        ui.label(
                                            RichText::new(format!("{done} / {total}"))
                                                .color(TEXT_MAIN)
                                                .size(13.0),
                                        );
                                        ui.label(
                                            RichText::new(format!("{:.0}% complete", pct * 100.0))
                                                .color(TEXT_DIM)
                                                .size(11.0),
                                        );
                                    },
                                );
                            });
                        });

                        ui.add_space(8.0);

                        // Full-width progress bar
                        let desired = egui::vec2(ui.available_width(), 8.0);
                        let (_, bar_rect) = ui.allocate_space(desired);
                        ui.painter().rect_filled(bar_rect, 4.0, PROGRESS_BG);
                        if pct > 0.0 {
                            let mut filled = bar_rect;
                            filled.max.x = bar_rect.min.x + bar_rect.width() * pct;
                            ui.painter().rect_filled(filled, 4.0, PROGRESS_FG);
                        }
                        // Stroke outline
                        ui.painter().rect_stroke(bar_rect, 4.0, Stroke::new(1.0, ACCENT_GOLD_DIM), egui::StrokeKind::Outside);
                    });

                ui.add_space(8.0);

                // Scrollable zone list
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .drag_to_scroll(false)
                    .show(ui, |ui| {
                        ui.add_space(4.0);

                        for (zi, zone) in act.zones.iter().enumerate() {
                            // Zone header
                            egui::Frame::new()
                                .fill(BG_ZONE)
                                .inner_margin(egui::Margin { left: 16, right: 16, top: 8, bottom: 8 })
                                .corner_radius(CornerRadius::same(4))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 62)))
                                .outer_margin(egui::Margin { left: 12, right: 12, top: 0, bottom: 4 })
                                .show(ui, |ui| {
                                    // Zone name
                                    let zone_done = zone.steps.iter().enumerate()
                                        .filter(|(si, _)| self.state.is_checked(act_idx, zi, *si))
                                        .count();
                                    let zone_total = zone.steps.len();
                                    let all_zone_done = zone_done == zone_total;

                                    ui.horizontal(|ui| {
                                        let name_col = if all_zone_done { ACCENT_GOLD_DIM } else { ACCENT_GOLD };
                                        ui.label(
                                            RichText::new(zone.name)
                                                .color(name_col)
                                                .size(13.0)
                                                .strong(),
                                        );
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(
                                                RichText::new(format!("{zone_done}/{zone_total}"))
                                                    .color(TEXT_DIM)
                                                    .size(10.0),
                                            );
                                        });
                                    });

                                    ui.add_space(4.0);

                                    // Steps
                                    for (si, step) in zone.steps.iter().enumerate() {
                                        let checked = self.state.is_checked(act_idx, zi, si);
                                        let is_warn = step.starts_with('⚠');

                                        // Two-column layout: [checkbox] [text block]
                                        // This keeps wrapped lines indented under the text, not the checkbox.
                                        ui.horizontal(|ui| {
                                            ui.add_space(4.0);

                                            // ── Checkbox ──
                                            let cb_size = egui::vec2(16.0, 16.0);
                                            let (cb_rect, cb_resp) = ui.allocate_exact_size(
                                                cb_size,
                                                egui::Sense::click(),
                                            );

                                            if cb_resp.clicked() {
                                                self.state.toggle(act_idx, zi, si);
                                                self.dirty = true;
                                            }

                                            let painter = ui.painter();
                                            let rounding = CornerRadius::same(3);
                                            let bg = if checked { CB_CHECKED_BG } else { CB_BG };
                                            painter.rect_filled(cb_rect, rounding, bg);
                                            let border_col = if cb_resp.hovered() { ACCENT_GOLD } else { CB_BORDER };
                                            painter.rect_stroke(cb_rect, rounding, Stroke::new(1.5, border_col), egui::StrokeKind::Outside);
                                            if checked {
                                                let p = cb_rect.min;
                                                let s = cb_rect.size();
                                                let p1 = egui::pos2(p.x + s.x * 0.18, p.y + s.y * 0.52);
                                                let p2 = egui::pos2(p.x + s.x * 0.42, p.y + s.y * 0.76);
                                                let p3 = egui::pos2(p.x + s.x * 0.82, p.y + s.y * 0.24);
                                                painter.line_segment([p1, p2], Stroke::new(2.0, CB_CHECK_MARK));
                                                painter.line_segment([p2, p3], Stroke::new(2.0, CB_CHECK_MARK));
                                            }

                                            ui.add_space(6.0); // gap between checkbox and text

                                            // ── Text (wraps within remaining width) ──
                                            let text_width = ui.available_width();
                                            ui.vertical(|ui| {
                                                ui.set_max_width(text_width);
                                                if checked {
                                                    // Strip tags so raw markup isn't visible
                                                    ui.label(
                                                        RichText::new(strip_tags(step))
                                                            .color(TEXT_DONE)
                                                            .size(13.0)
                                                            .strikethrough(),
                                                    );
                                                } else if is_warn {
                                                    ui.label(RichText::new(*step).color(WARN_COLOR).size(13.0));
                                                } else {
                                                    // Render colored segments inline with wrapping
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.spacing_mut().item_spacing.x = 0.0;
                                                        let segments = parse_segments(step);
                                                        for seg in &segments {
                                                            let col = seg.color.unwrap_or(TEXT_MAIN);
                                                            ui.label(RichText::new(&seg.text).color(col).size(13.0));
                                                        }
                                                    });
                                                }
                                            });
                                        });
                                    }
                                });

                            ui.add_space(4.0);
                        }

                        ui.add_space(20.0);
                    });
                } // end else (checklist view)
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.close_overlay();
    }
}
