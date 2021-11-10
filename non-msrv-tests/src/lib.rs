#[test]
fn ui() {
    use trybuild::TestCases;

    TestCases::new().compile_fail("tests/ui/*.rs");
    #[cfg(feature = "zeroize")]
    TestCases::new().compile_fail("tests/ui/zeroize/*.rs");
}
