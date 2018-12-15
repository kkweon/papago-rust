//! Papago module
//!
//! # Example
//!
//! ```rust
//! # use papago::*;
//! let auth = config::get_auth_from_env().unwrap();
//! let payload = PapagoReq {
//!     source: Lang::En,
//!     target: Lang::Ko,
//!     text: "HELLO WORLD".to_owned(),
//! };
//! let resp = send_request(NMT_URL, &auth, payload).unwrap();
//! assert_eq!(resp, "안녕 세계".to_owned());
//! ```
#[macro_use]
extern crate serde_derive;


/// Configuration module. It contains Auth
pub mod config;

/// Actual Papago API wrapper module.
pub mod api;

/// CLI Arguments
pub mod argparse;

pub use self::api::NMT_URL;
pub use self::api::SMT_URL;
pub use self::api::send_request;
pub use self::api::Lang;
pub use self::api::PapagoReq;
