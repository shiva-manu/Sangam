//! Startup banner for the Sangam headless CLI.
//!
//! Prints a brief header to stdout the first time the node process starts,
//! making it easy to confirm which version of the runtime is running without
//! having to inspect the binary directly.

/// Project version, sourced at compile time from `Cargo.toml` so the
/// banner can never drift out of sync with the package metadata.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Print the Sangam startup banner to stdout.
///
/// Outputs the project name and the compile-time version string surrounded
/// by a decorative border. This function is called once at process startup,
/// before any async tasks are spawned, so the banner always appears at the
/// very top of the terminal output.
///
/// # Example output
/// ```text
/// =======================================================
///                      Sangam v0.1.0
/// =======================================================
/// ```
pub fn show_banner() {
    println!("\n=======================================================");
    println!("                     Sangam v{}", VERSION);
    println!("=======================================================\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_banner_does_not_panic() {
        // Smoke test: just make sure printing the banner is safe to call.
        show_banner();
    }

    #[test]
    fn version_starts_with_a_digit() {
        // env!() resolves at compile time — if it doesn't match Cargo.toml,
        // the build itself would fail. This test mostly documents intent
        // and guards against someone re-introducing a hard-coded literal
        // by asserting the value looks like a real semver string.
        assert!(VERSION.chars().next().unwrap().is_ascii_digit());
        assert!(VERSION.contains('.'), "expected semver, got `{}`", VERSION);
    }
}
