//! Strongly-typed manifest structs used by Amazon S3 in Rust
//!
//! #### Example
//! ```ignore
//!     let data = include_bytes!("manifest.json");
//!
//!     // slice to struct
//!     let parsed: InventoryManifest = serde_json::from_slice(data).unwrap();
//!     // struct to string
//!     let output: String = serde_json::to_string(&parsed).unwrap();
//! ```

/// definitions for S3 Inventory
#[cfg(feature = "inventory")]
pub mod inventory;
