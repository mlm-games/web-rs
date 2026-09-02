# web-rs
Rust bindings to make WASM more tolerable.

## Unstable API
Some crates use unstable `web_sys` APIs and you may need to set `--cfg=web_sys_unstable_apis` when compiling.
For more information, see the [web-sys docs](https://rustwasm.github.io/wasm-bindgen/web-sys/unstable-apis.html).

There's a few ways to set this depending on the environment:
- [Cargo Config](./cargo/config.toml) via `rustflags`
- [Github Action](.github/workflows/pr.yml) via `GITHUB_ENV`
- [docs.rs](./web-codecs/Cargo.toml) via `package.metadata.docs.rs`

## Maintenance stance

We intentionally **do not** maintain wrappers where a well-maintained external crate already exists. Prefer the external crate and treat the `web-*` crate as a thin compat layer or deprecated:

| Crate | External canonical alternative | When to keep this crate |
|---|---|---|
| `web-async` | `web-time` (`std::time` on `wasm32-unknown-unknown` via `Performance.now()`), `wasmtimer` / `gloo-timers` (`tokio::time` on `setTimeout`), `tokio_with_wasm` (full `tokio` alias), `wasm-bindgen-futures::spawn_local` | Only for `Lock` (`Arc<Mutex>` ↔ `Rc<RefCell>`) / `FuturesExt` shims |
| `web-streams` | [`wasm-streams`](https://crates.io/crates/wasm-streams) (`2.7M/mo`, `futures::Stream`/`Sink` ↔ `ReadableStream`/`WritableStream`/`TransformStream`) | Only for backwards compat; new code → `wasm-streams` |
| `web-message` / `web-message-derive` | [`serde-wasm-bindgen`](https://crates.io/crates/serde-wasm-bindgen) (`0.6`, official `wasm-bindgen` + `serde`) | Only if you need **transferable zero-copy** (`ArrayBuffer`/`VideoFrame`/`MessagePort` via `postMessage(..., [transfer])`); otherwise `serde-wasm-bindgen` |
| `web-codecs` | _(none - unique)_ | **Keep** - channel-based `VideoEncoder`/`VideoDecoder`/`AudioEncoder`/`AudioDecoder` wrapper has no external equivalent |

If you only need `Instant::now()`/`SystemTime::now()` in the browser, use `web-time` directly (re-exports `std::time` off-`wasm`). Do **not** add a custom `web-async::time` module based on `wasmtimer` - depend on `wasmtimer`/`gloo-timers` instead if you need async timers.

## web-codecs
[web-codecs](./web-codecs) provides a wrapper around the [WebCodecs API](https://developer.mozilla.org/en-US/docs/Web/API/WebCodecs_API).

The callbacks have been replaced with a channel-like API.
For example, the `VideoEncoder` is split into a `VideoEncoder` for input and a `VideoEncoded` for output. This crate is the primary maintained crate in this repo.

## web-streams
[web-streams](./web-streams) provides a wrapper around the [Streams API](https://developer.mozilla.org/en-US/docs/Web/API/Streams_API).

This API is annoyingly untyped when using web_sys.
This library handles the casting for you as well as providing guard-rails around the API (ex. closing on Drop).

> **Deprecated for new code:** prefer [`wasm-streams`](https://crates.io/crates/wasm-streams) for full `Stream`/`Sink`/`TransformStream` bridging.

## web-async
[web-async](./web-async) is a minimal `Send`-agnostic shim (`spawn`, `Lock`, `FuturesExt`).

> Prefer `web-time` / `wasmtimer` / `wasm-bindgen-futures` for time and spawning. See `web-async/src/lib.rs` docs.

## web-message
[web-message](./web-message) provides `postMessage` with transferable support.

> Prefer `serde-wasm-bindgen` unless you need zero-copy `ArrayBuffer`/`VideoFrame` transfers. See `web-message/src/lib.rs` docs.