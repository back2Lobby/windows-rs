// Bindings generated by `windows-bindgen` 0.48.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub type IUnknown = *mut ::core::ffi::c_void;
pub type BOOL = i32;
::windows_targets::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : IUnknown) -> BOOL);
