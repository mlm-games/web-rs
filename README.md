# web-rs
`web-codecs` only. `web-workers` (despite name, also covers `std::thread`/`sync`/`time`/`message`) is used for threading/message where needed.

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

## web-workers
Despite the name, [`web-workers`](https://crates.io/crates/web-workers) (`web-workers="0.3"` with `features=["message"]` in this workspace) is a drop-in `std::thread`/`sync`/`time`/`message` for `wasm32-unknown-unknown` (via `DedicatedWorker` + `SharedArrayBuffer`/`Atomics`). Use it wherever you would use `std::thread::spawn`, `Mutex`, `Instant`, `tokio::time` or `postMessage` transfers on WASM. Native targets re-export `std`. `web-message`'s `Message` trait + `derive(Message)` zero-copy logic ( `ArrayBuffer`/`VideoFrame` via `transfer` array, `Vec<T>`/`Option<T>`, `ts-rs` parity) maps directly to `web_workers::web::message::MessageSend::send(&mut transfer) -> RawMessage` - add that `derive` macro to `web-workers` (TODO in `web-workers/src/lib.rs:3` `Add MessageSend macro`) and `web-message` becomes redundant. See `web-workers` docs for `has_spawn_support()`/`join_async` guards and COOP/COEP requirements.

## Removed crates - use external alternatives

| Removed crate | Use instead |
|---|---|
| `web-async` (`spawn`, `Lock`, `FuturesExt`, `time`) | `web-workers` for `thread`/`sync`/`time` (`web-time` for `std::time` alone, `wasmtimer`/`gloo-timers` for `tokio::time`, `wasm-bindgen-futures::spawn_local` otherwise) |
| `web-streams` | [`wasm-streams`](https://crates.io/crates/wasm-streams) (`futures::Stream`/`Sink` ↔ `ReadableStream`/`WritableStream`/`TransformStream`) |
| `web-message`/`web-message-derive` | `web-workers` `web::message::MessageSend` (`features=["message"]` enables `VideoFrame`/`AudioData`/etc.) + upstream TODO `MessageSend` derive (port `web-message-derive` `src/lib.rs:5` `derive(Message)` logic); until then `serde-wasm-bindgen` for non-transfer, or vendor `web-message` if you need `derive` today |
