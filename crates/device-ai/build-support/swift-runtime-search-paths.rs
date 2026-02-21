use std::{path::Path, process::Command};

/// Link the Swift runtime search paths needed by binaries that load Swift code.
pub fn link_swift_runtime_search_paths() {
    if let Ok(output) = Command::new("xcrun")
        .args(["--show-sdk-path", "--sdk", "macosx"])
        .output()
    {
        if output.status.success() {
            let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-link-search=native={sdk_path}/usr/lib/swift");
        }
    }

    if let Ok(output) = Command::new("xcrun")
        .args(["--toolchain", "default", "--find", "swiftc"])
        .output()
    {
        if output.status.success() {
            let swiftc_path = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if let Some(toolchain_dir) = Path::new(&swiftc_path)
                .parent()
                .and_then(|path| path.parent())
            {
                let swift_lib = toolchain_dir.join("lib/swift/macosx");

                if swift_lib.exists() {
                    println!("cargo:rustc-link-search=native={}", swift_lib.display());
                    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", swift_lib.display());
                }
            }
        }
    }

    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");
}
