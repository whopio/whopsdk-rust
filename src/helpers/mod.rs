//! Hand-written helpers Fern cannot generate.
//!
//! Fern generates from OpenAPI paths, so anything that is not an HTTP call has
//! no path to be generated from. Everything here depends only on third-party
//! crates and the standard library, never on generated client code, so it
//! survives the client being replaced. `.fernignore` keeps the files;
//! `src/lib.rs` is kept with them because it is generated and is the only place
//! a Rust module can be declared.

pub mod verify_webhook;

pub use verify_webhook::{verify_webhook, WebhookVerificationError, TOLERANCE_SECONDS};
