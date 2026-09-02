//! Wrapper around the [Streams API](https://developer.mozilla.org/en-US/docs/Web/API/Streams_API).
//!
//! **Maintenance note:** For new code prefer [`wasm-streams`](https://crates.io/crates/wasm-streams)
//! (`MattiasBuelens/wasm-streams`, `2.7M/mo`, bridges `futures::Stream`/`Sink` ↔ `ReadableStream`/`WritableStream`/`TransformStream` with `AsyncRead`/`tee`).
//! That crate is the canonical, well-maintained bridging layer (used by `deno_web`, `gloo` ecosystem).
//! This crate (`web-streams`) is retained for backwards compatibility only: it provides a thin typed
//! `Reader`/`Writer`/`TypedWriter` with `Drop`-based `release_lock` and optional `tokio::io::{AsyncRead,AsyncWrite}` impls,
//! but does not aim to cover `TransformStream` or the full spec. For `TransformStream`/`Sink`/`Stream` conversions, use `wasm-streams`.

mod error;
mod promise;
mod reader;
mod writer;

pub use error::*;
pub(crate) use promise::*;
pub use reader::*;
pub use writer::*;
