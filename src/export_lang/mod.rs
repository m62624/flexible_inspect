#[cfg(any(feature = "python-lib", feature = "wasm"))]
mod core_extension;
#[cfg(feature = "python-lib")]
mod python_version;
#[cfg(feature = "wasm")]
mod wasm_version;
