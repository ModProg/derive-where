use quote::quote;
use syn::Result;

use super::test_derive;

#[test]
fn zeroize() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Zeroize)]
			struct Test<T>(core::marker::PhantomData<T>);
		},
		quote! {
			impl<T> ::zeroize::Zeroize for Test<T>
			{
				fn zeroize(&mut self) {
					match self {
						Test(ref mut __0) => {
							__0.zeroize();
						}
					}
				}
			}
		},
	)
}

#[test]
fn zeroize_drop() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Zeroize(drop); T)]
			struct Test<T, U>(T, core::marker::PhantomData<U>);
		},
		quote! {
			impl<T, U> ::zeroize::Zeroize for Test<T, U>
			where T: ::zeroize::Zeroize
			{
				fn zeroize(&mut self) {
					match self {
						Test(ref mut __0, ref mut __1) => {
							__0.zeroize();
							__1.zeroize();
						}
					}
				}
			}

			impl<T, U> ::core::ops::Drop for Test<T, U>
			where T: ::zeroize::Zeroize
			{
				fn drop(&mut self) {
					::zeroize::Zeroize::zeroize(self);
				}
			}
		},
	)
}

#[test]
fn zeroize_crate() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Zeroize(crate = "zeroize_"); T)]
			struct Test<T>(T);
		},
		quote! {
			impl<T> zeroize_::Zeroize for Test<T>
			where T: zeroize_::Zeroize
			{
				fn zeroize(&mut self) {
					match self {
						Test(ref mut __0) => {
							__0.zeroize();
						}
					}
				}
			}
		},
	)
}

#[test]
fn zeroize_drop_crate() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Zeroize(drop, crate = "zeroize_"); T)]
			struct Test<T>(T);
		},
		quote! {
			impl<T> zeroize_::Zeroize for Test<T>
			where T: zeroize_::Zeroize
			{
				fn zeroize(&mut self) {
					match self {
						Test(ref mut __0) => {
							__0.zeroize();
						}
					}
				}
			}

			impl<T> ::core::ops::Drop for Test<T>
			where T: zeroize_::Zeroize
			{
				fn drop(&mut self) {
					zeroize_::Zeroize::zeroize(self);
				}
			}
		},
	)
}

#[test]
fn zeroize_crate_drop() -> Result<()> {
	test_derive(
		quote! {
			#[derive_where(Zeroize(crate = "zeroize_", drop); T)]
			struct Test<T>(T);
		},
		quote! {
			impl<T> zeroize_::Zeroize for Test<T>
			where T: zeroize_::Zeroize
			{
				fn zeroize(&mut self) {
					match self {
						Test(ref mut __0) => {
							__0.zeroize();
						}
					}
				}
			}

			impl<T> ::core::ops::Drop for Test<T>
			where T: zeroize_::Zeroize
			{
				fn drop(&mut self) {
					zeroize_::Zeroize::zeroize(self);
				}
			}
		},
	)
}
