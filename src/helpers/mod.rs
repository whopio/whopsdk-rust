//! Hand-written helpers Fern cannot generate.
//!
//! Fern generates from OpenAPI paths, so anything that is not an HTTP call has
//! no path to be generated from. Everything here depends only on third-party
//! crates and the standard library, never on generated client code, so it
//! survives the client being replaced. `.fernignore` keeps the files;
//! the module declaration in generated `src/lib.rs` follows the existing OIDC
//! declaration through Fern Replay.

pub mod verify_webhook;

pub use verify_webhook::{verify_webhook, WebhookVerificationError, TOLERANCE_SECONDS};
