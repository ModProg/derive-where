// TODO: Remove this constraint when `Span`s stabilize.
#[cfg(feature = "nightly")]
#[test]
fn ui() {
	use trybuild::TestCases;

	TestCases::new().compile_fail("tests/ui/*.rs");
	#[cfg(feature = "zeroize")]
	TestCases::new().compile_fail("tests/ui/zeroize/*.rs");
	#[cfg(not(feature = "zeroize"))]
	TestCases::new().compile_fail("tests/ui/not-zeroize/*.rs");
}

#[test]
fn readme_sync() {
	use readme_sync::{assert_sync, CMarkDocs, CMarkReadme, Config, Package};

	let package = Package::from_path("..".into()).unwrap();
	let config = Config::from_package_docs_rs_features(&package);
	let readme = CMarkReadme::from_package(&package).unwrap();
	let docs = CMarkDocs::from_package_and_config(&package, &config).unwrap();

	let readme = readme
		.remove_badges_paragraph()
		.remove_documentation_section();

	let docs = docs
		.increment_heading_levels()
		.add_package_title()
		.remove_codeblock_rust_test_tags()
		.use_default_codeblock_rust_tag()
		.remove_hidden_rust_code();

	match std::env::var("WRITE_README") {
		Ok(value) if value == "1" => {
			let mut file = String::new();
			pulldown_cmark_to_cmark::cmark(docs.iter_events(), &mut file, None).unwrap();
			std::fs::write("../README.md", file).unwrap();
		}
		_ => (),
	}

	assert_sync(&readme, &docs);
}
