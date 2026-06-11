//! `egui_kittest_mcp` — an MCP server that drives egui apps over the `egui_inspection`
//! protocol, plus the building blocks to embed or extend it.
//!
//! The `kittest-mcp` binary (`src/main.rs`) is a thin wrapper around [`server::run`]. The
//! library surface exists so other servers can reuse the machinery — notably `re_mcp`,
//! which reuses [`Server`]'s tools but tunnels the inspection protocol through rerun's gRPC
//! `MessageProxyService` instead of a local socket:
//!
//! - [`Bridge::from_transport`] builds a bridge over any async byte transport.
//! - [`AppState::install_bridge`] injects that bridge so the existing tools drive it.
//! - [`Server`] implements `rmcp::ServerHandler`, so a host can delegate `list_tools` /
//!   `call_tool` to it for the egui tool set and layer its own tools on top.

pub mod bridge;
pub mod server;
pub mod tools;
pub mod tree;

pub use bridge::{Bridge, PeerInfo, SharedState, StateSnapshot};
pub use tools::{AppState, Server};
