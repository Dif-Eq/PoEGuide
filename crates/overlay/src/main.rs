#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;

/// Ensures the overlay window has WS_EX_LAYERED | WS_EX_TRANSPARENT so it stays
/// click-through. Safe to call repeatedly — it's idempotent (OR-assigns the flags).
#[cfg(target_os = "windows")]
pub(crate) fn set_passthrough(title: &str) {
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
    // Refuse to start if the tracker isn't running — this executable is only
    // meant to be launched via the "Open Overlay" button in the tracker UI.
    #[cfg(target_os = "windows")]
    {
        use windows::{
            core::w,
            Win32::{
                Foundation::HWND,
                System::Threading::{OpenMutexW, SYNCHRONIZATION_ACCESS_RIGHTS},
                UI::WindowsAndMessaging::{MessageBoxW, MB_ICONINFORMATION, MB_OK},
            },
        };
        // SYNCHRONIZE = 0x00100000 — minimum access needed to open an existing mutex
        let handle = unsafe { OpenMutexW(SYNCHRONIZATION_ACCESS_RIGHTS(0x00100000), false, w!("Local\\PoE2GuideTracker")) };
        if handle.is_err() {
            unsafe {
                MessageBoxW(
                    HWND(std::ptr::null_mut()),
                    w!("Please launch the overlay using the 'Open Overlay' button in the PoE2 Campaign Guide tracker."),
                    w!("PoE2 Overlay"),
                    MB_OK | MB_ICONINFORMATION,
                );
            }
            return Ok(());
        }
    }

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

    // Apply passthrough style shortly after the window is created.
    // The app also re-applies it periodically in case the OS resets the flags.
    #[cfg(target_os = "windows")]
    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_millis(500));
        set_passthrough("PoE2 Overlay");
    });

    eframe::run_native(
        "PoE2 Overlay",
        options,
        Box::new(|_cc| Ok(Box::new(app::OverlayApp::new()))),
    )
}
