fn main() {
    // Declare custom cfg for FoundationModels SDK detection.
    println!("cargo::rustc-check-cfg=cfg(has_foundation_models)");

    #[cfg(target_os = "macos")]
    macos::init();
}

#[cfg(target_os = "macos")]
#[path = "build-support/swift-runtime-search-paths.rs"]
mod swift_runtime;

#[cfg(target_os = "macos")]
mod macos {
    use super::swift_runtime;
    use std::path::Path;
    use std::process::Command;

    pub fn init() {
        // 1. Explicitly link frameworks and Swift Concurrency library
        // This ensures the linker knows it's a dependency.
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Translation");
        println!("cargo:rustc-link-lib=swift_Concurrency");

        // 2. Add the Swift runtime to the RPATH
        // This is the most important step. It tells the loader to check
        // /usr/lib/swift at runtime, where the shared cache is mapped.
        println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

        // 3. Search path for the SDK (for compilation)
        // Helps the linker find the .tbd (stub) files in the SDK.
        if let Ok(output) = Command::new("xcrun").args(["--show-sdk-path"]).output() {
            let sdk_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("cargo:rustc-link-search=native={}/usr/lib/swift", sdk_path);
        }

        compile_swift_bridges();
    }

    /// Compile the Swift bridges on macOS (FoundationModels LLM and Translation frameworks).
    ///
    /// This compiles Swift sources into an object file and links them into a static archive.
    fn compile_swift_bridges() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let llm_src = format!("{manifest_dir}/src/swift/llm_bridge.swift");
        let translation_src = format!("{manifest_dir}/src/swift/translation_bridge.swift");

        println!("cargo:rerun-if-changed=src/swift/llm_bridge.swift");
        println!("cargo:rerun-if-changed=src/swift/translation_bridge.swift");

        if !Path::new(&llm_src).exists() {
            println!(
                "cargo:warning=src/swift/llm_bridge.swift not found, skipping Swift bridge compilation"
            );
            return;
        }

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let obj_path = format!("{out_dir}/llm_bridge.o");

        let target = std::env::var("TARGET").unwrap_or_default();
        let arch = if target.contains("aarch64") {
            "arm64"
        } else {
            "x86_64"
        };
        let target_triple = format!("{arch}-apple-macos26.0");

        let mut swift_sources = vec![llm_src];
        if Path::new(&translation_src).exists() {
            swift_sources.push(translation_src);
        }

        let mut cmd = Command::new("swiftc");
        cmd.arg("-c");
        for src in &swift_sources {
            cmd.arg(src);
        }
        cmd.args([
            "-o",
            &obj_path,
            "-target",
            &target_triple,
            "-O",
            "-whole-module-optimization",
            "-parse-as-library",
        ]);

        let result = cmd.output();

        match result {
            Ok(output) if output.status.success() => {
                let lib_path = format!("{out_dir}/libllm_bridge.a");
                let ar_result = Command::new("ar")
                    .args(["rcs", &lib_path, &obj_path])
                    .output();

                match ar_result {
                    Ok(ar_out) if ar_out.status.success() => {
                        println!("cargo:rustc-link-search=native={out_dir}");
                        println!("cargo:rustc-link-lib=static=llm_bridge");
                        println!("cargo:rustc-link-lib=framework=FoundationModels");
                        println!("cargo:rustc-cfg=has_foundation_models");
                        link_swift_runtime();
                    }
                    _ => {
                        println!("cargo:warning=Failed to create static archive from Swift object");
                        println!("cargo:warning=LLM features will be stubbed");
                    }
                }
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("cargo:warning=Swift bridge compilation failed: {stderr}");
                println!("cargo:warning=LLM features will be stubbed (requires macOS 26 SDK)");
            }
            Err(e) => {
                println!("cargo:warning=swiftc not found or failed to run: {e}");
                println!("cargo:warning=LLM features will be stubbed");
            }
        }
    }

    /// Link the Swift runtime libraries needed for the compiled Swift object.
    fn link_swift_runtime() {
        swift_runtime::link_swift_runtime_search_paths();
        println!("cargo:rustc-link-lib=dylib=swiftCore");
        println!("cargo:rustc-link-lib=dylib=swiftFoundation");
        println!("cargo:rustc-link-lib=dylib=swift_Concurrency");
    }
}
