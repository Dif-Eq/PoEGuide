#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;

#[cfg(target_os = "windows")]
fn set_passthrough(title: &str) {
    use windows::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetWindowLongW, SetWindowLongW,
        GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT,
    };

    let wide: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();
    unsafe {
        if let Ok(hwnd) = FindWindowW(None, windows::core::PCWSTR(wide.as_ptr())) {
            let style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            SetWindowLongW(
                hwnd,
                GWL_EXSTYLE,
                style | WS_EX_LAYERED.0.cast_signed() | WS_EX_TRANSPARENT.0.cast_signed(),
            );
        }
    }
}

fn main() -> eframe::Result<()> {
    let config = shared::config::Config::load();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("PoE2 Overlay")
            .with_inner_size([420.0, 300.0])
            .with_min_inner_size([280.0, 120.0])
            .with_position([config.overlay_x, config.overlay_y])
            .with_always_on_top()
            .with_decorations(false)
            .with_transparent(true),
        persist_window: false,
        ..Default::default()
    };

    // Spawn a thread to apply passthrough style shortly after the window is created
    #[cfg(target_os = "windows")]
    std::thread::spawn(|| {
        // Give the window time to be created and registered with the OS
        std::thread::sleep(std::time::Duration::from_millis(500));
        set_passthrough("PoE2 Overlay");
    });

    eframe::run_native(
        "PoE2 Overlay",
        options,
        Box::new(|_cc| Ok(Box::new(app::OverlayApp::new()))),
    )
}
