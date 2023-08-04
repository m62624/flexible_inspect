#[cfg(not(tarpaulin_include))]
#[cfg(any(feature = "python-lib", feature = "wasm"))]
pub mod core_extension;

#[cfg(not(tarpaulin_include))]
#[cfg(feature = "python-lib")]
pub mod python_version;

#[cfg(not(tarpaulin_include))]
#[cfg(feature = "wasm")]
pub mod wasm_version;
