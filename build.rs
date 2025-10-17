#[cfg(windows)]
use std::path::Path;
#[cfg(windows)]
use std::process::Command;

fn main() {
    #[cfg(windows)]
    {
        let rc_path = Path::new("res/librius.rc");

        // Rebuild only if the .rc file changes
        println!("cargo:rerun-if-changed=res/librius.rc");

        if rc_path.exists() {
            // Compile the .rc file into a .res file using windres
            // For MSVC toolchains, ensure MinGW is available in PATH
            let result = Command::new("windres")
                .env("PATH", "X:\\mingw64\\bin") // Adjust path if MinGW is installed elsewhere
                .args(["res/librius.rc", "-O", "coff", "-o", "res/librius.res"])
                .status();

            match result {
                Ok(status) if status.success() => {
                    // Link the compiled resource file into the final executable
                    println!("cargo:rustc-link-arg=res/librius.res");
                }
                Ok(status) => {
                    println!(
                        "cargo:warning=windres exited with non-zero status: {:?}",
                        status.code()
                    );
                }
                Err(_) => {
                    println!("cargo:warning=windres not found or failed to execute.");
                }
            }
        } else {
            println!("cargo:warning=res/librius.rc not found, skipping icon embedding.");
        }
    }

    // Non-Windows platforms: no resource embedding required.
}
