// Desktop platform implementation (egui backend)

// Re-export existing desktop functionality
// (This is a placeholder for organizing platform-specific code)

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub use crate::app;

#[cfg(all(not(target_arch = "wasm32"), feature = "desktop"))]
pub use crate::app_docking;

#[cfg(test)]
mod tests {
    #[test]
    fn test_desktop_platform() {
        // Platform tests will go here
    }
}
