fn main() {
    // Embed the icon into the .exe on Windows (shows in Explorer, taskbar, alt-tab)
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
