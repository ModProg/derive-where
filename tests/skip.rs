#![allow(clippy::clone_on_copy)]

#[macro_use]
mod util;
#[path = "skip/debug.rs"]
mod debug;
#[path = "skip/field.rs"]
mod field;
#[path = "skip/field_trait.rs"]
mod field_trait;
#[path = "skip/struct_.rs"]
mod struct_;
#[path = "skip/struct_trait.rs"]
mod struct_trait;
#[path = "skip/variant.rs"]
mod variant;
#[path = "skip/variant_trait.rs"]
mod variant_trait;
