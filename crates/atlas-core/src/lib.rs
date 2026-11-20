//! `atlas-core`: backends and background work — filesystem, search, cloud,
//! extensions, tag DB, watchers, jobs. Depends on `atlas-model`; never on the UI.
//!
//! Moved here from the legacy `src/{fs,cloud,ext,search,util}` trees (Phase J); the
//! binary re-exports each as `crate::{fs,cloud,ext,search,util}` via shims.
// reason: the sync `std::fs` ban in clippy.toml protects the GPUI foreground
// thread in atlas-ui. This crate's file I/O runs on worker threads
// (`spawn_blocking`, sidecars, background jobs), so blocking calls are expected.
#![allow(clippy::disallowed_methods)]

pub mod cleanup;
pub mod convert;
pub mod ext;
pub mod fs;
pub mod license;
pub mod search;
pub mod space_lens;
pub mod update;
pub mod usage;
pub mod util;
pub mod vault;

#[cfg(feature = "provider-host")]
pub mod cloud;
