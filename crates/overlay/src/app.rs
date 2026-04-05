#![allow(clippy::uninlined_format_args)]

use eframe::egui::{self, Color32, RichText, Stroke, CornerRadius};
use global_hotkey::{
    GlobalHotKeyManager, GlobalHotKeyEvent,
    hotkey::{HotKey, Modifiers, Code},
};
use shared::data::{all_acts, Act};
use shared::save::SaveState;
use shared::config::Config;
use std::time::SystemTime;

// ─── Tag stripping ────────────────────────────────────────────────────────────

fn strip_tags(input: &str) -> String {
    let tags = ["[b]","[/b]","[i]","[/i]","[s]","[/s]","[q]","[/q]","[z]","[/z]","[o]","[/o]"];
    let mut out = input.to_string();
    for tag in &tags {
        out = out.replace(tag, "");
    }
    out
}

// ─── Key parsing ─────────────────────────────────────────────────────────────

fn parse_code(key: &str) -> Option<Code> {
    match key {
        "F1"  => Some(Code::F1),  "F2"  => Some(Code::F2),  "F3"  => Some(Code::F3),
        "F4"  => Some(Code::F4),  "F5"  => Some(Code::F5),  "F6"  => Some(Code::F6),
        "F7"  => Some(Code::F7),  "F8"  => Some(Code::F8),  "F9"  => Some(Code::F9),
        "F10" => Some(Code::F10), "F11" => Some(Code::F11), "F12" => Some(Code::F12),
        "A" => Some(Code::KeyA), "B" => Some(Code::KeyB), "C" => Some(Code::KeyC),
        "D" => Some(Code::KeyD), "E" => Some(Code::KeyE), "F" => Some(Code::KeyF),
        "G" => Some(Code::KeyG), "H" => Some(Code::KeyH), "I" => Some(Code::KeyI),
        "J" => Some(Code::KeyJ), "K" => Some(Code::KeyK), "L" => Some(Code::KeyL),
        "M" => Some(Code::KeyM), "N" => Some(Code::KeyN), "O" => Some(Code::KeyO),
        "P" => Some(Code::KeyP), "Q" => Some(Code::KeyQ), "R" => Some(Code::KeyR),
        "S" => Some(Code::KeyS), "T" => Some(Code::KeyT), "U" => Some(Code::KeyU),
        "V" => Some(Code::KeyV), "W" => Some(Code::KeyW), "X" => Some(Code::KeyX),
        "Y" => Some(Code::KeyY), "Z" => Some(Code::KeyZ),
        _ => None,
    }
}

fn hotkey_from_config(hk: &shared::config::Hotkey) -> Option<HotKey> {
    let code = parse_code(&hk.key)?;
    let mut mods = Modifiers::empty();
    if hk.ctrl  { mods |= Modifiers::CONTROL; }
    if hk.shift { mods |= Modifiers::SHIFT;   }
    if hk.alt   { mods |= Modifiers::ALT;     }
    Some(HotKey::new(Some(mods), code))
}

// ─── Colors ───────────────────────────────────────────────────────────────────

// const BG_PANEL: Color32        = Color32::from_rgba_premultiplied(18, 22, 28, 210);
// const BG_HEADER: Color32       = Color32::from_rgba_premultiplied(12, 15, 20, 220);
const ACCENT_GOLD: Color32     = Color32::from_rgb(210, 185, 95);
// const ACCENT_GOLD_DIM: Color32 = Color32::from_rgb(120, 105, 50);
// const CRIMSON: Color32         = Color32::from_rgb(160, 30, 30);
const TEXT_MAIN: Color32       = Color32::from_rgb(210, 215, 220);
const TEXT_DIM: Color32        = Color32::from_rgb(120, 130, 140);
const TEXT_DONE: Color32       = Color32::from_rgb(65, 72, 80);
const TEXT_CURRENT: Color32    = Color32::from_rgb(20, 18, 12);
const STEP_HIGHLIGHT: Color32  = Color32::from_rgba_premultiplied(160, 130, 45, 180);

const STEPS_AHEAD: usize = 4;

// ─── Registered hotkey IDs ───────────────────────────────────────────────────

struct HotkeyIds {
    advance: u32,
    undo:    u32,
    toggle:  u32,
}

// ─── App ──────────────────────────────────────────────────────────────────────

pub struct OverlayApp {
    acts:              Vec<Act>,
    state:             SaveState,
    config:            Config,
    frame_counter:     u32,
    visible:           bool,
    manager:          GlobalHotKeyManager,
    hotkey_ids:        Option<HotkeyIds>,
    config_mtime:      Option<SystemTime>,
    pending_reposition: bool,
}

impl OverlayApp {
    pub fn new() -> Self {
        let config = Config::load();
        let manager = GlobalHotKeyManager::new().expect("Failed to create hotkey manager");
        let hotkey_ids = Self::register_hotkeys(&manager, &config);
        let config_mtime = Config::last_modified();

        Self {
            acts: all_acts(),
            state: SaveState::load(),
            config,
            frame_counter: 0,
            visible: true,
            manager,
            hotkey_ids,
            config_mtime,
            pending_reposition: false,
        }
    }

    fn register_hotkeys(manager: &GlobalHotKeyManager, config: &Config) -> Option<HotkeyIds> {
        let hk_advance = hotkey_from_config(&config.hotkey_advance)?;
        let hk_undo    = hotkey_from_config(&config.hotkey_undo)?;
        let hk_toggle  = hotkey_from_config(&config.hotkey_toggle)?;

        manager.register(hk_advance).ok()?;
        manager.register(hk_undo).ok()?;
        manager.register(hk_toggle).ok()?;

        Some(HotkeyIds {
            advance: hk_advance.id(),
            undo:    hk_undo.id(),
            toggle:  hk_toggle.id(),
        })
    }

    fn reload_hotkeys(&mut self) {
        // Unregister old hotkeys by re-creating them from old config
        if let Some(ids) = &self.hotkey_ids {
            if let Some(hk) = hotkey_from_config(&self.config.hotkey_advance) {
                if hk.id() == ids.advance { let _ = self.manager.unregister(hk); }
            }
            if let Some(hk) = hotkey_from_config(&self.config.hotkey_undo) {
                if hk.id() == ids.undo { let _ = self.manager.unregister(hk); }
            }
            if let Some(hk) = hotkey_from_config(&self.config.hotkey_toggle) {
                if hk.id() == ids.toggle { let _ = self.manager.unregister(hk); }
            }
        }
        self.config = Config::load();
        self.hotkey_ids = Self::register_hotkeys(&self.manager, &self.config);
        self.config_mtime = Config::last_modified();
        self.pending_reposition = true;
    }

    fn poll(&mut self) {
        self.frame_counter += 1;

        // Poll save state frequently for responsive step syncing
        if self.frame_counter.is_multiple_of(6) {
            self.state = SaveState::load();
        }

        // Poll config less frequently — only needed for hotkey/position/opacity changes
        if self.frame_counter >= 30 {
            self.frame_counter = 0;
            let mtime = Config::last_modified();
            if mtime != self.config_mtime {
                self.reload_hotkeys();
            }
        }
    }

    fn advance(&mut self) {
        if let Some((ai, zi, si)) = self.state.first_unchecked(&self.acts) {
            self.state.toggle(ai, zi, si);
            self.state.save();
        }
    }

    fn undo(&mut self) {
        let mut last: Option<(usize, usize, usize)> = None;
        for (ai, act) in self.acts.iter().enumerate() {
            for (zi, zone) in act.zones.iter().enumerate() {
                for (si, _) in zone.steps.iter().enumerate() {
                    if self.state.is_checked(ai, zi, si) {
                        last = Some((ai, zi, si));
                    }
                }
            }
        }
        if let Some((ai, zi, si)) = last {
            self.state.toggle(ai, zi, si);
            self.state.save();
        }
    }

    fn process_hotkeys(&mut self) {
        // Collect matching actions first to avoid borrow conflict
        let mut do_advance = false;
        let mut do_undo    = false;
        let mut do_toggle  = false;

        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state != global_hotkey::HotKeyState::Pressed {
                continue;
            }
            if let Some(ids) = &self.hotkey_ids {
                if event.id == ids.advance { do_advance = true; }
                if event.id == ids.undo    { do_undo    = true; }
                if event.id == ids.toggle  { do_toggle  = true; }
            }
        }

        // Now act — no active borrows of self.hotkey_ids
        if do_advance { self.advance(); }
        if do_undo    { self.undo();    }
        if do_toggle  { self.visible = !self.visible; }
    }
}

impl eframe::App for OverlayApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.poll();
        self.process_hotkeys();

        // Scroll opacity removed — overlay is click/scroll passthrough
        // Opacity can be adjusted via config if needed in future

        if !self.visible {
            egui::CentralPanel::default()
                .frame(egui::Frame::new().fill(Color32::TRANSPARENT))
                .show(ctx, |_ui| {});
            ctx.request_repaint();
            return;
        }

        let mut style = (*ctx.style()).clone();
        style.visuals.override_text_color = Some(TEXT_MAIN);
        style.visuals.window_fill = Color32::TRANSPARENT;
        style.visuals.panel_fill  = Color32::TRANSPARENT;
        style.spacing.item_spacing = egui::vec2(0.0, 4.0);
        ctx.set_style(style);

        // Position is set at startup via with_position() in main.rs
        // and updated here whenever config changes
        if self.pending_reposition {
            self.pending_reposition = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                egui::pos2(self.config.overlay_x, self.config.overlay_y)
            ));
        }

        let next_steps = self.state.next_unchecked(&self.acts, STEPS_AHEAD + 1);
        let advance_label = self.config.hotkey_advance.display();
        let undo_label    = self.config.hotkey_undo.display();
        let toggle_label  = self.config.hotkey_toggle.display();

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let panel_alpha = (self.config.opacity * 255.0) as u8;
                let bg         = Color32::from_rgba_unmultiplied(18, 22, 28, panel_alpha);
                let header_bg  = Color32::from_rgba_unmultiplied(12, 15, 20, panel_alpha);

                egui::Frame::new()
                    .fill(bg)
                    .corner_radius(CornerRadius::same(8))
                    .stroke(Stroke::new(1.0, Color32::from_rgba_unmultiplied(160, 30, 30, panel_alpha)))
                    .inner_margin(egui::Margin::same(0))
                    .show(ui, |ui| {
                        // ── Header / drag handle ──────────────────────────
                        let header_resp = egui::Frame::new()
                            .fill(header_bg)
                            .corner_radius(CornerRadius { nw: 8, ne: 8, sw: 0, se: 0 })
                            .inner_margin(egui::Margin { left: 10, right: 10, top: 6, bottom: 6 })
                            .show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    let act_name = next_steps.first()
                                        .map_or("Complete!", |(ai, _, _)| self.acts[*ai].name);

                                    // Act name + step counter
                                    ui.label(
                                        RichText::new(act_name)
                                            .color(ACCENT_GOLD)
                                            .size(13.0)
                                            .strong(),
                                    );

                                    // Progress counter in header
                                    if let Some((ai, _, _)) = next_steps.first() {
                                        let (done, total) = self.state.act_progress(*ai, &self.acts[*ai]);
                                        #[allow(clippy::cast_precision_loss)]
                                        let pct = if total > 0 { done as f32 / total as f32 } else { 0.0 };
                                        ui.label(
                                            RichText::new(format!("  {done}/{total}  ({:.0}%)", pct * 100.0))
                                                .color(TEXT_DIM)
                                                .size(10.0),
                                        );
                                    }

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(
                                            RichText::new(
                                                format!("{advance_label} next  {undo_label} undo  {toggle_label} hide")
                                            )
                                            .color(TEXT_DONE)
                                            .size(9.0),
                                        );
                                    });
                                });

                                // Thin progress bar under the header text
                                if let Some((ai, _, _)) = next_steps.first() {
                                    let (done, total) = self.state.act_progress(*ai, &self.acts[*ai]);
                                    #[allow(clippy::cast_precision_loss)]
                                    let pct = if total > 0 { done as f32 / total as f32 } else { 0.0 };
                                    ui.add_space(4.0);
                                    let desired = egui::vec2(ui.available_width(), 3.0);
                                    let (_, bar_rect) = ui.allocate_space(desired);
                                    let progress_trough = Color32::from_rgba_unmultiplied(40, 30, 20, panel_alpha);
                                    let progress_fill   = Color32::from_rgba_unmultiplied(160, 30, 30, panel_alpha);
                                    ui.painter().rect_filled(bar_rect, 2.0, progress_trough);
                                    if pct > 0.0 {
                                        let mut filled = bar_rect;
                                        filled.max.x = bar_rect.min.x + bar_rect.width() * pct;
                                        ui.painter().rect_filled(filled, 2.0, progress_fill);
                                    }
                                }
                            })
                            .response;

                        if header_resp.interact(egui::Sense::drag()).dragged() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                        }

                        // ── Steps ─────────────────────────────────────────
                        egui::Frame::new()
                            .inner_margin(egui::Margin { left: 10, right: 10, top: 6, bottom: 8 })
                            .show(ui, |ui| {
                                if next_steps.is_empty() {
                                    ui.label(
                                        RichText::new("All steps complete! Enjoy the endgame.")
                                            .color(ACCENT_GOLD)
                                            .size(13.0),
                                    );
                                } else {
                                    for (idx, (ai, zi, si)) in next_steps.iter().enumerate() {
                                        let step_text = strip_tags(self.acts[*ai].zones[*zi].steps[*si]);
                                        let is_current = idx == 0;

                                        if is_current {
                                            egui::Frame::new()
                                                .fill(STEP_HIGHLIGHT)
                                                .corner_radius(CornerRadius::same(4))
                                                .inner_margin(egui::Margin { left: 6, right: 6, top: 3, bottom: 3 })
                                                .show(ui, |ui| {
                                                    ui.set_min_width(ui.available_width());
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.spacing_mut().item_spacing.x = 0.0;
                                                        ui.label(RichText::new("▶ ")
                                                            .color(Color32::from_rgb(80, 55, 10))
                                                            .size(11.0));
                                                        let zone_name = self.acts[*ai].zones[*zi].name;
                                                        ui.label(RichText::new(format!("[{zone_name}]  "))
                                                            .color(Color32::from_rgb(60, 45, 10))
                                                            .size(11.0));
                                                        ui.label(RichText::new(&step_text)
                                                            .color(TEXT_CURRENT)
                                                            .size(14.0));
                                                    });
                                                });
                                        } else {
                                            egui::Frame::new()
                                                .inner_margin(egui::Margin { left: 6, right: 6, top: 2, bottom: 2 })
                                                .show(ui, |ui| {
                                                    ui.set_min_width(ui.available_width());
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.spacing_mut().item_spacing.x = 0.0;
                                                        ui.label(RichText::new("  ").size(11.0));
                                                        let zone_name = self.acts[*ai].zones[*zi].name;
                                                        ui.label(RichText::new(format!("[{zone_name}]  "))
                                                            .color(TEXT_DONE)
                                                            .size(10.0));
                                                        ui.label(RichText::new(&step_text)
                                                            .color(TEXT_DIM)
                                                            .size(11.0));
                                                    });
                                                });
                                        }

                                        ui.add_space(2.0);
                                    }
                                }
                            });
                    });
            });

        ctx.request_repaint();
    }
}
