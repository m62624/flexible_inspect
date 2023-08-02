#[cfg(any(feature = "python-lib", feature = "wasm"))]
pub mod core_extension;

#[cfg(feature = "python-lib")]
pub mod python_version;

#[cfg(feature = "wasm")]
pub mod wasm_version;
