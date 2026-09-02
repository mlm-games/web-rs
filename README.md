# web-rs
Rust bindings to make WASM more tolerable. Maintained crates: `web-codecs` (unique) + `web-message`/`web-message-derive` (zero-copy). Others removed in favor of external crates. `web-workers` (despite name, also covers `std::thread`/`sync`/`time`) is used wherever threading is needed.

## Unstable API
Some crates use unstable `web_sys` APIs and you may need to set `--cfg=web_sys_unstable_apis` when compiling.
For more information, see the [web-sys docs](https://rustwasm.github.io/wasm-bindgen/web-sys/unstable-apis.html).

There's a few ways to set this depending on the environment:
- [Cargo Config](./cargo/config.toml) via `rustflags`
- [Github Action](.github/workflows/pr.yml) via `GITHUB_ENV`
- [docs.rs](./web-codecs/Cargo.toml) via `package.metadata.docs.rs`

## web-codecs
[web-codecs](./web-codecs) provides a wrapper around the [WebCodecs API](https://developer.mozilla.org/en-US/docs/Web/API/WebCodecs_API).

The callbacks have been replaced with a channel-like API.
For example, the `VideoEncoder` is split into a `VideoEncoder` for input and a `VideoEncoded` for output. No well-maintained external alternative.

## web-message
[web-message](./web-message) provides `postMessage` with zero-copy transferables (`ArrayBuffer`/`VideoFrame`/`ImageBitmap` etc. via `postMessage(..., [transfer])`) and `#[derive(Message)]` with `ts-rs` parity. Use `serde-wasm-bindgen` if you don't need zero-copy; otherwise `web-message` is the maintained path. Works alongside `web-workers` (for `web_workers::spawn` threads, enable `web-workers` `message` feature; for generic `Worker` use `web-message` directly).

## web-workers
Despite the name, [`web-workers`](https://crates.io/crates/web-workers) (`web-workers="0.3"` with `features=["message"]` in this workspace) is a drop-in `std::thread`/`sync`/`time` for `wasm32-unknown-unknown` (via `DedicatedWorker` + `SharedArrayBuffer`/`Atomics`). Use it wherever you would use `std::thread::spawn`, `Mutex`, `Instant` or `tokio::time` on WASM. Native targets re-export `std`. See `web-workers` docs for `has_spawn_support()`/`join_async` guards and COOP/COEP requirements.

## Removed crates - use external alternatives

| Removed crate | Use instead |
|---|---|
| `web-async` (`spawn`, `Lock`, `FuturesExt`, `time`) | `web-time` for `std::time`, `wasmtimer`/`gloo-timers` for `tokio::time`, `web-workers` for `thread`/`sync`/`time`, `wasm-bindgen-futures::spawn_local` otherwise |
| `web-streams` | [`wasm-streams`](https://crates.io/crates/wasm-streams) (`futures::Stream`/`Sink` ↔ `ReadableStream`/`WritableStream`/`TransformStream`) |
