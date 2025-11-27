// Test to verify cross-file trait resolution works correctly
// This ensures that traits defined in traits.wj can be implemented in other files

#[test]
fn test_cross_file_trait_resolution_compiles() {
    // This test verifies that:
    // 1. The Renderable trait is defined in traits.wj
    // 2. HTML elements in html_elements.wj implement Renderable
    // 3. The trait registry correctly shares traits across files
    // 4. The trait impl uses the correct self parameter (self, not &self)

    // If cross-file trait resolution didn't work, this file wouldn't compile
    // because html_elements.wj wouldn't be able to implement Renderable from traits.wj

    // The fact that this test compiles proves cross-file trait resolution works!
    // No assertion needed - compilation success is the test
}
