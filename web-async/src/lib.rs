//! Async helpers and utilities for WASM.
//!
//! **Maintenance note:** This crate is intentionally minimal. For functionality already covered by
//! well-maintained external crates, prefer those instead of expanding this crate:
//! - `std::time::{Instant,SystemTime}` on `wasm32-unknown-unknown` → [`web-time`](https://crates.io/crates/web-time) (`web-time = "1"` drop-in `std::time` via `Performance.now()`/`Date.now()`).
//! - `tokio::time::{sleep,timeout,interval}` on browser → [`wasmtimer`](https://crates.io/crates/wasmtimer) or [`gloo-timers`](https://crates.io/crates/gloo-timers) (`wasmtimer::tokio` mirrors `tokio::time` on `setTimeout`), or [`tokio_with_wasm`](https://crates.io/crates/tokio_with_wasm) for a full `tokio` alias.
//! - `spawn` → `wasm-bindgen-futures::spawn_local` / `tokio::task::spawn` directly, or `tokio_with_wasm::alias` if you need a unified `tokio` API.
//! This crate only retains `Lock`/`FuturesExt`/`spawn` shims that have no single canonical external replacement.

mod futures;
mod lock;
mod spawn;

pub use futures::*;
pub use lock::*;
pub use spawn::*;
