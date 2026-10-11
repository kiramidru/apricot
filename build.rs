use std::collections::HashSet;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(target_os = "linux")]
    {
        let mut rpath_dirs: HashSet<PathBuf> = HashSet::new();

        // 1. OpenGL / Vulkan graphics drivers on NixOS
        for gl_dir in &["/run/opengl-driver/lib", "/run/opengl-driver-32/lib"] {
            let p = Path::new(gl_dir);
            if p.exists() {
                rpath_dirs.insert(p.to_path_buf());
            }
        }

        // 2. Libraries needed by winit, eframe, wgpu, wayland, and x11
        let required_libs = [
            "libwayland-client.so.0",
            "libwayland-cursor.so.0",
            "libwayland-egl.so.0",
            "libxkbcommon.so.0",
            "libGL.so.1",
            "libEGL.so.1",
            "libvulkan.so.1",
            "libX11.so.6",
            "libXcursor.so.1",
            "libXi.so.6",
            "libXrandr.so.2",
        ];

        // 2a. Search existing LD_LIBRARY_PATH
        if let Ok(ld_path) = std::env::var("LD_LIBRARY_PATH") {
            for entry in ld_path.split(':') {
                if !entry.is_empty() {
                    let p = Path::new(entry);
                    if p.exists() {
                        rpath_dirs.insert(p.to_path_buf());
                    }
                }
            }
        }

        // 2b. Search system profiles
        for base in &["/run/current-system/sw/lib", "/etc/profiles/per-user"] {
            let p = Path::new(base);
            if p.exists() {
                rpath_dirs.insert(p.to_path_buf());
            }
        }

        // 2c. On NixOS, search /nix/store for any missing required libraries
        let nix_store = Path::new("/nix/store");
        if nix_store.exists() {
            if let Ok(entries) = std::fs::read_dir(nix_store) {
                let mut found_libs: HashSet<&str> = HashSet::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                    // Check common packages
                    if file_name.contains("-wayland-")
                        || file_name.contains("-libxkbcommon-")
                        || file_name.contains("-libGL-")
                        || file_name.contains("-vulkan-loader-")
                        || file_name.contains("-libX11-")
                    {
                        let lib_dir = path.join("lib");
                        if lib_dir.is_dir() {
                            for target_lib in required_libs {
                                if !found_libs.contains(target_lib) && lib_dir.join(target_lib).exists() {
                                    rpath_dirs.insert(lib_dir.clone());
                                    found_libs.insert(target_lib);
                                }
                            }
                        }
                    }

                    if found_libs.len() == required_libs.len() {
                        break;
                    }
                }
            }
        }

        // 3. Emit rustc link arguments for each directory
        for dir in &rpath_dirs {
            if let Some(dir_str) = dir.to_str() {
                println!("cargo:rustc-link-arg=-Wl,-rpath,{}", dir_str);
            }
        }
    }
}
