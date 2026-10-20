# FigCraft on BrowserOS

This branch builds FigCraft for [BrowserOS](https://github.com/Progate/packages/tree/main/packages/browser-os)
(`wasm32-wasip1-threads`). The window, input, CPU painting and the file dialogs live in `@progate/browser-fig`
(on top of `packages/browser-egui`); this fork only holds what the FigCraft crates need to build and run there.

## Changes

- **`cfg(target_arch = "wasm32")` means the web build.** Every such `cfg` in `crates/` became
  `cfg(all(target_arch = "wasm32", target_os = "unknown"))`. The web build (`web-sys`, `wasm-bindgen`) stays
  as it was; on WASI the crates take the native path (`std::fs`, `std::time`, threads), as they do on Linux.
- **`parking_lot_core` waits with std's park / unpark on WASI** (`browseros/crates/parking_lot_core`, patched in
  through `[patch.crates-io]`). Upstream's wasm thread parker panics as soon as a lock is contended unless the
  nightly feature is on; the std of `wasm32-wasip1-threads` implements park / unpark with
  `memory.atomic.wait32` / `notify`, so `thread_parker/wasi.rs` waits on that. The rest is crates.io's 0.9.12.
- **Dropped files are read by path on WASI** (`crates/ui-egui/src/lib.rs::dropped_bytes`). egui's `DroppedFile`
  trait only offers `bytes_async` on wasm32; on BrowserOS a dropped file has a real path, so `std::fs::read` is
  enough.

## License

As the rest of FigCraft (MIT OR Apache-2.0). `browseros/crates/parking_lot_core` keeps its own license
(MIT OR Apache-2.0, see its `LICENSE-*`).
