#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod icon;

fn main() -> eframe::Result<()> {
    // Hold a named mutex for our lifetime so the overlay can verify the tracker is running.
    #[cfg(target_os = "windows")]
    let _tracker_mutex = unsafe {
        use windows::Win32::System::Threading::CreateMutexW;
        use windows::core::w;
        CreateMutexW(None, false, w!("Local\\PoE2GuideTracker")).ok()
    };
    let icon_data = egui::IconData {
        rgba: icon::ICON_RGBA.to_vec(),
        width: icon::ICON_WIDTH,
        height: icon::ICON_HEIGHT,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("PoE2 Campaign Guide")
            .with_inner_size([860.0, 680.0])
            .with_min_inner_size([600.0, 400.0])
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "PoE2 Campaign Guide",
        options,
        Box::new(|cc| Ok(Box::new(app::GuideApp::new(cc)))),
    )
}
