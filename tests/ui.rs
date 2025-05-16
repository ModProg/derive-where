// TODO: Remove nightly constraint when `Span`s stabilize.
#![cfg(all(feature = "nightly", not(miri)))]

#[test]
fn ui() {
	use trybuild::TestCases;

	TestCases::new().compile_fail("tests/ui/*.rs");
	#[cfg(feature = "serde")]
	TestCases::new().compile_fail("tests/ui/serde/*.rs");
	#[cfg(not(feature = "serde"))]
	TestCases::new().compile_fail("tests/ui/not-serde/*.rs");
	#[cfg(not(feature = "zeroize"))]
	TestCases::new().compile_fail("tests/ui/not-zeroize/*.rs");
	#[cfg(feature = "zeroize")]
	TestCases::new().compile_fail("tests/ui/zeroize/*.rs");
	#[cfg(all(feature = "zeroize", not(feature = "zeroize-on-drop")))]
	TestCases::new().compile_fail("tests/ui/not-zeroize-on-drop/*.rs");
	#[cfg(feature = "zeroize-on-drop")]
	TestCases::new().compile_fail("tests/ui/zeroize-on-drop/*.rs");
}
