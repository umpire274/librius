#[cfg(target_os = "windows")]
fn main() {
    use winresource::WindowsResource;

    // Assicurati che res/librius.ico esista
    let mut res = WindowsResource::new();
    res.set_icon("res/librius.ico")
        .set("FileDescription", "Librius CLI")
        .set("ProductName", "Librius")
        .set("OriginalFilename", "librius.exe")
        .set("FileVersion", env!("CARGO_PKG_VERSION"))
        .set("ProductVersion", env!("CARGO_PKG_VERSION"))
        .compile()
        .expect("Failed to embed icon resource");
}

#[cfg(not(target_os = "windows"))]
fn main() {}
