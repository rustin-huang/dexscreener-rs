//! # dexscreener-rs
//!
//! A Rust client library for interacting with the [DexScreener API](https://docs.dexscreener.com/api/reference).
//! This library provides a type-safe, ergonomic way to access DexScreener's data about decentralized
//! exchange trading pairs across multiple blockchains.
//!
//! ## Features
//!
//! - Complete support for all DexScreener API endpoints
//! - Fully asynchronous API with Tokio runtime support
//! - Type-safe return values with comprehensive error handling
//! - Flexible configuration options
//! - Robust parsing of various data formats
//!
//! ## Basic Usage
//!
//! ```no_run
//! use dexscreener_rs::DexScreenerClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new client instance
//!     let client = DexScreenerClient::new();
//!
//!     // Get information for a specific trading pair
//!     let pair_response = client.get_pairs_by_chain_and_address(
//!         "ethereum",
//!         "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
//!     ).await?;
//!
//!     if let Some(pair) = pair_response.pairs.first() {
//!         println!("Pair: {} - {}", pair.base_token.symbol, pair.quote_token.symbol);
//!         println!("Price: ${:.2}", pair.price_usd.unwrap_or(0.0));
//!         println!("24h Volume: ${:.2}", pair.volume.h24);
//!     }
//!
//!     // Get all pairs containing a specific token
//!     let token_response = client.get_pair_by_token_address(
//!         "ethereum",
//!         "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"  // WETH
//!     ).await?;
//!
//!     println!("Found {} pairs containing the token", token_response.pairs.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! All API methods return a `Result<T, DexScreenerError>` where `T` is the appropriate response type.
//! The `DexScreenerError` enum provides detailed information about what went wrong, including
//! API errors, network issues, and parsing problems.
//!
//! ## Rate Limiting
//!
//! The DexScreener API has rate limits that vary by endpoint. These are documented in each method.
//! When a rate limit is exceeded, the API will return an error, which this library will propagate
//! as a `DexScreenerError::ApiError`.

// Module declarations
pub mod client;
pub mod errors;
pub mod models;

// Public exports
pub use client::DexScreenerClient;
pub use errors::DexScreenerError;
pub use models::{
    BaseToken, Liquidity, PairResponse, PairTransactionCounts, PriceChangePeriods, SearchResponse,
    TokenPair, TransactionCount, VolumeChangePeriods,
};

/// API version used by this crate
pub const API_VERSION: &str = "latest";

/// Base URL for DexScreener API
pub const API_BASE_URL: &str = "https://api.dexscreener.com";
