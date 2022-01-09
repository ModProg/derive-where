mod basic;
mod bound;
mod enum_;
mod misc;
mod skip;
mod use_case;
#[cfg(feature = "zeroize")]
mod zeroize;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

fn test_derive(input: TokenStream, expected: TokenStream) -> Result<()> {
	let left = crate::derive_where_internal(input.clone())?.to_string();
    // FIXME this currently means, that input_without_derive_where_attributes is not actually
    // tested.
	let item = crate::input_without_derive_where_attributes(syn::parse2(input)?);
	let right = quote! {#item #expected}.to_string();

	assert_eq!(left, right);
	Ok(())
}

fn compiles(input: TokenStream) -> Result<()> {
	crate::derive_where_internal(input).map(|_| ())
}
