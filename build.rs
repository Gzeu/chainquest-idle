use std::env;
fn main() {
    println!("Build script for linking native libs if needed.");
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "linux" {
        // Placeholder for any platform-specific flags
        println!("cargo:rustc-link-arg=-Wl,--as-needed");
    }
}
