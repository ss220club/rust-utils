#![forbid(unsafe_op_in_unsafe_fn)]

#[macro_use]
mod byond;
#[allow(dead_code)]
mod error;

pub mod transliteration;
pub mod regexp;

#[cfg(not(target_pointer_width = "32"))]
compile_error!("rust-utils must be compiled for a 32-bit target");
