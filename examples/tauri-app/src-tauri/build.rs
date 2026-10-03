fn main() {
    tauri_build::build();

    // The device-ai-apis plugin links Swift code (for FoundationModels LLM support).
    // Swift runtime dylibs use @rpath install names, so we must add the toolchain's
    // Swift lib directory to the binary's rpath. This cannot be set from a library
    // crate's build.rs (cargo:rustc-link-arg doesn't propagate from rlibs).
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("xcrun")
            .args(["--toolchain", "default", "--find", "swiftc"])
            .output()
        {
            if output.status.success() {
                let swiftc = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if let Some(toolchain) =
                    std::path::Path::new(&swiftc).parent().and_then(|p| p.parent())
                {
                    let swift_lib = toolchain.join("lib/swift/macosx");
                    if swift_lib.exists() {
                        println!(
                            "cargo:rustc-link-arg=-Wl,-rpath,{}",
                            swift_lib.display()
                        );
                    }
                }
            }
        }

        // Also add /usr/lib/swift as fallback rpath
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

        // Embed Info.plist into __TEXT,__info_plist for macOS TCC privacy checks (e.g. Speech recognition)
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
        let info_plist = std::path::Path::new(&manifest_dir).join("Info.plist");
        if info_plist.exists() {
            println!(
                "cargo:rustc-link-arg=-Wl,-sectcreate,__TEXT,__info_plist,{}",
                info_plist.display()
            );
        }
    }
}
