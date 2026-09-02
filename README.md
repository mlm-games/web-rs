# web-rs

Single-crate workspace for [`wasodecs`](./wasodecs), which is a channel-based wrapper around the [WebCodecs API](https://developer.mozilla.org/en-US/docs/Web/API/WebCodecs_API).

Threading and `postMessage` transfers are out of scope here; use [`web-workers`](https://crates.io/crates/web-workers) (`features = ["message"]`) instead.

## Unstable API

Some crates use unstable `web-sys` APIs and require `--cfg=web_sys_unstable_apis`.

See the [web-sys unstable APIs docs](https://rustwasm.github.io/wasm-bindgen/web-sys/unstable-apis.html).

Configured via:

- [Cargo config](.cargo/config.toml) (`build.rustflags`)
- [CI](.github/workflows/pr.yml) (`RUSTFLAGS` in `GITHUB_ENV`)
- [docs.rs](wasodecs/Cargo.toml) (`package.metadata.docs.rs`)

## wasodecs

[wasodecs](./wasodecs) replaces WebCodecs callbacks with a channel-like API.
For example, `VideoEncoder` (input) is split from `VideoEncoded` (output). See crate docs for details.

## Removed crates

Prefer these external crates over the former `web-*` crates in this repo:

| Removed crate | Use instead |
|---|---|
| `web-async` (`spawn`, `Lock`, `FuturesExt`, `time`) | `web-workers` for `thread`/`sync`/`time` (`web-time` for `std::time` alone, `wasmtimer`/`gloo-timers` for `tokio::time`, `wasm-bindgen-futures::spawn_local` otherwise) |
| `web-streams` | [`wasm-streams`](https://crates.io/crates/wasm-streams) (`futures::Stream`/`Sink` ↔ `ReadableStream`/`WritableStream`/`TransformStream`) |
| `web-message` / `web-message-derive` | `web-workers` `web::message::MessageSend` (`features = ["message"]`) if derive is also needed. |
