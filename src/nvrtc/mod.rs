//! Wrappers around the [Nvidia Runtime Compilation (nvrtc) API](https://docs.nvidia.com/cuda/nvrtc/index.html),
//! in three levels. See crate documentation for description of each.
//!
//! Call [compile_ptx()] or [compile_ptx_with_opts()].

pub mod result;
pub mod safe;
#[allow(warnings)]
pub mod sys;

pub use safe::*;

lazy_static::lazy_static! {
    static ref NVRTC: sys::Nvrtc = unsafe {sys::Nvrtc::new(libloading::library_filename("nvrtc")).expect("Could not load nvrtc library")};
}
