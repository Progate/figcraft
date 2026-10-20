//! Live sessions over the network for FigCraft (milestone M9, steps 2 and 4
//! of `docs/collab-protocol.md`): the WebSocket transport, the session host the desktop app embeds,
//! invite links, and presence relay. It carries the collaboration core's messages
//! ([`figcraft_collab::message`]) and runs its [`Server`](figcraft_collab::Server) as the host's
//! authority.
//!
//! The app installs a [`NetLink`] into the engine session (`session.live.install(…)`); the
//! engine's `collab.*` commands drive it and the UI reads the engine's live state. Network
//! threads wake the UI through a [`Waker`] only when something arrives; nothing polls.
//!
//! The browser build's [`NetLink`] joins sessions through the browser's WebSocket API (`web-sys`);
//! hosting needs the desktop app. Its connection logic (`driver.rs`) is target-independent and
//! tested natively.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

#[cfg(any(all(target_arch = "wasm32", target_os = "unknown"), test))]
mod driver;
pub mod inbox;
pub mod invite;
mod join;
pub mod limits;
mod sessions;
pub mod throttle;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod guest;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod host;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod hub;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod link;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod links;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod manage;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod net;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod tls;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub mod token;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod web;

pub use inbox::{Inbox, Waker};
pub use invite::{Endpoint, Invite};
pub use limits::NetLimits;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub use link::{DEFAULT_PORT, NetLink, lan_ip};
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use web::NetLink;

#[cfg(test)]
mod code_tests;
#[cfg(test)]
mod driver_tests;
#[cfg(test)]
mod invite_tests;
#[cfg(test)]
mod links_tests;
#[cfg(test)]
mod loopback_tests;
#[cfg(test)]
mod session_tests;
#[cfg(test)]
mod sessions_tests;
#[cfg(test)]
mod tls_tests;
