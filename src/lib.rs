//! # spacetrack‑rs
//!
//! Lightweight, typed Rust client for **Space‑Track.org**. Handles authentication,
//! query building, and parsing of results into strongly‑typed structs.
//!
//! ```rust,no_run
//! use spacetrack_rs::{SpaceTrackClient, Format};
//! use spacetrack_rs::models::{SatCatEntry, GpEntry};
//!
//! fn main() -> Result<(), spacetrack_rs::Error> {
//!     let user = std::env::var("ST_USER")?;
//!     let pass = std::env::var("ST_PASS")?;
//!     let client = SpaceTrackClient::login(&user, &pass)?;
//!
//!     // all sats launched in last 7 days
//!     let recent: Vec<SatCatEntry> = client.satcat()
//!         .filter("LAUNCH", ">now-7")
//!         .filter("CURRENT", "Y")
//!         .execute()?;
//!     println!("{} recent sats", recent.len());
//!
//!     // ISS latest GP element
//!     let iss: GpEntry = client.gp()
//!         .filter("NORAD_CAT_ID", "25544")
//!         .execute_one()?;
//!     println!("ISS epoch {}", iss.EPOCH);
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod format;
pub mod models;
mod query;

pub use client::SpaceTrackClient;
pub use error::Error;
pub use format::Format;
