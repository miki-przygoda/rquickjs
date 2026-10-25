#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::uninlined_format_args)]
#![no_std]

use ::core::ptr;

/// Common error message for converting between C `size_t` and Rust `usize`;
pub const SIZE_T_ERROR: &str =
    "conversion between C type 'size_t' and Rust type 'usize' overflowed.";

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!(concat!("bindings/", bindings_env!("TARGET"), ".rs"));

/// Functions declared by hand because the bundled bindings predate them.
///
/// They are re-exported with a glob, which an item of the same name shadows: when bindgen
/// generates the bindings from the current `quickjs.h` (the `bindgen` feature when
/// cross-compiling, or `update-bindings`), its declaration is the one in use and these go
/// unused, so every target and feature set sees exactly one of each.
mod manual {
    use super::JSContext;

    unsafe extern "C" {
        /// Turns off compiling source text in `ctx` (see `JS_DisableEval` in `quickjs.h`).
        pub fn JS_DisableEval(ctx: *mut JSContext);
    }
}

#[allow(unused_imports)]
pub use manual::*;

#[cfg(target_pointer_width = "64")]
include!("inlines/ptr_64.rs");

#[cfg(target_pointer_width = "32")]
include!("inlines/ptr_32_nan_boxing.rs");

include!("inlines/common.rs");
