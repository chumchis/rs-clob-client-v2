//! Polymarket Data API client and types.
//!
//! **Feature flag:** `data` (required to use this module)
//!
//! This module provides a client for interacting with the Polymarket Data API,
//! which offers HTTP endpoints for querying user positions, trades, activity,
//! market holders, open interest, volume data, and leaderboards.
//!
//! # Overview
//!
//! The Data API is a read-only HTTP API that provides access to Polymarket data.
//! It is separate from the CLOB (Central Limit Order Book) API which handles trading.
//!
//! ## Available Endpoints
//!
//! Polymarket retired Data API v1 on 2026-11-02. The core user routes are on v2:
//! `{data, pagination}` envelope, `snake_case` rows, opaque cursor paging (no `offset`).
//!
//! | Endpoint | Description |
//! |----------|-------------|
//! | `/` | Health check |
//! | `/v2/positions` | Positions for a user, by lifecycle `status` (incl. `CLOSED`) |
//! | `/v2/trades` | Trades for a user, markets, events or the global tape |
//! | `/v2/activity` | On-chain activity for a user |
//! | `/v2/value` | Total value of a user's positions |
//!
//! Still on retired v1 routes and marked `#[deprecated]` until migrated:
//! `holders`, `closed_positions` (use `positions` with `PositionStatus::Closed`),
//! `traded`, `open_interest`, `live_volume`, `leaderboard`, `builder_leaderboard`,
//! `builder_volume`.
//!
//! # Example
//!
//! ```no_run
//! use polymarket_client_sdk_v2::types::address;
//! use polymarket_client_sdk_v2::data::{Client, types::request::PositionsRequest};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a client with the default endpoint
//! let client = Client::default();
//!
//! // Build a request for user positions
//! let request = PositionsRequest::builder()
//!     .user(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
//!     .build();
//!
//! // Fetch the first page of open positions
//! let page = client.positions(&request).await?;
//!
//! for position in &page.data {
//!     println!("{}: {} tokens at ${:.2}",
//!         position.title,
//!         position.current_size,
//!         position.current_value
//!     );
//! }
//! // `page.next_cursor()` is `Some` while more pages remain.
//! # Ok(())
//! # }
//! ```
//!
//! # API Base URL
//!
//! The default API endpoint is `https://data-api.polymarket.com`.

pub mod client;
pub mod types;

pub use client::Client;
