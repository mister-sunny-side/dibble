pub mod types;
pub mod error;

#[cfg(feature = "server")]
pub mod client;
#[cfg(feature = "server")]
pub mod daemon;
#[cfg(feature = "server")]
pub mod mock_client;
#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "server")]
pub use client::ResyClient;
#[cfg(feature = "server")]
pub use mock_client::MockResyClient;

pub use error::{ResyError, ResyResult};
pub use types::*;
