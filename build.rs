//! Build script that runs before compilation
//!
//! This demonstrates how to pass build-time information to your application.
//! The environment variables set here can be accessed in your code using the
//! `env!()` macro.

use std::env;

fn main() {
    // Get the current timestamp
    let build_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Set environment variable that will be available at compile time
    // This can be accessed in your code with: env!("BUILD_TIMESTAMP")
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", build_time);

    // Set another variable with the build profile (debug or release)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

    // Tell cargo to re-run this script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    println!("Build script executed at: {}", build_time);
}
