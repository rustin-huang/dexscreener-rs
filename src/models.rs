use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

/// Helper function to deserialize string or number to f64.
///
/// This handles cases where the API might return a numeric value as either
/// a JSON number or a string.
pub fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(f64),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => f64::from_str(&s).map_err(serde::de::Error::custom),
        StringOrNumber::Number(n) => Ok(n),
    }
}

/// Helper function to deserialize optional string or number to Option<f64>.
///
/// This handles cases where the API might return a numeric value as either
/// a JSON number, a string, or not include the field at all.
pub fn deserialize_optional_string_or_number<'de, D>(
    deserializer: D,
) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OptionalStringOrNumber {
        String(String),
        Number(f64),
        None,
    }

    match OptionalStringOrNumber::deserialize(deserializer)? {
        OptionalStringOrNumber::String(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                f64::from_str(&s)
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
        }
        OptionalStringOrNumber::Number(n) => Ok(Some(n)),
        OptionalStringOrNumber::None => Ok(None),
    }
}

/// Helper function to deserialize Unix timestamp (milliseconds) to DateTime<Utc>.
///
/// This handles cases where the API might return a timestamp as either
/// a number (Unix timestamp in milliseconds) or an RFC 3339 formatted string.
pub fn deserialize_timestamp_to_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TimestampOrString {
        Timestamp(i64),
        String(String),
        None,
    }

    match TimestampOrString::deserialize(deserializer)? {
        TimestampOrString::Timestamp(ts) => {
            // Convert milliseconds to seconds and nanoseconds
            let secs = ts / 1000;
            let nsecs = ((ts % 1000) * 1_000_000) as u32;
            Ok(Some(Utc.timestamp_opt(secs, nsecs).unwrap()))
        }
        TimestampOrString::String(s) => {
            if s.is_empty() {
                return Ok(None);
            }

            // Try parsing as RFC3339 string
            match DateTime::parse_from_rfc3339(&s) {
                Ok(dt) => Ok(Some(dt.with_timezone(&Utc))),
                Err(_) => {
                    // Try parsing as timestamp string
                    match s.parse::<i64>() {
                        Ok(ts) => {
                            let secs = ts / 1000;
                            let nsecs = ((ts % 1000) * 1_000_000) as u32;
                            Ok(Some(Utc.timestamp_opt(secs, nsecs).unwrap()))
                        }
                        Err(e) => Err(serde::de::Error::custom(format!(
                            "Failed to parse datetime: {}",
                            e
                        ))),
                    }
                }
            }
        }
        TimestampOrString::None => Ok(None),
    }
}

/// Represents basic information about a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseToken {
    /// The blockchain address of the token
    pub address: String,
    /// The full name of the token
    pub name: String,
    /// The token's symbol/ticker
    pub symbol: String,
}

/// Statistics about transactions (buys and sells).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionCount {
    /// Number of buy transactions in the time period
    pub buys: i64,
    /// Number of sell transactions in the time period
    pub sells: i64,
}

/// Transaction statistics for various time periods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairTransactionCounts {
    /// Transactions in the last 5 minutes
    pub m5: TransactionCount,
    /// Transactions in the last 1 hour
    pub h1: TransactionCount,
    /// Transactions in the last 6 hours
    pub h6: TransactionCount,
    /// Transactions in the last 24 hours
    pub h24: TransactionCount,
}

/// Represents numerical data over different time periods.
///
/// This is used for various metrics like trading volume and price changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriodsFloat {
    /// Data for the last 5 minutes
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub m5: f64,
    /// Data for the last 1 hour
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub h1: f64,
    /// Data for the last 6 hours
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub h6: f64,
    /// Data for the last 24 hours
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub h24: f64,
}

/// Trading volume changes in USD for different time periods.
pub type VolumeChangePeriods = TimePeriodsFloat;

/// Price change percentages for different time periods.
pub type PriceChangePeriods = TimePeriodsFloat;

/// Represents the liquidity information for a trading pair.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Liquidity {
    /// Liquidity value in USD
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_string_or_number")]
    pub usd: Option<f64>,
    /// Amount of base token in the liquidity pool
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub base: f64,
    /// Amount of quote token in the liquidity pool
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub quote: f64,
}

/// Represents a trading pair on a decentralized exchange.
///
/// This contains comprehensive information about a trading pair, including
/// tokens, prices, volumes, and liquidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    /// The ID of the blockchain where this pair exists
    #[serde(rename = "chainId")]
    pub chain_id: String,
    /// The ID of the decentralized exchange (e.g., "uniswap", "sushiswap")
    #[serde(rename = "dexId")]
    pub dex_id: String,
    /// URL to view this pair on DexScreener
    pub url: String,
    /// The contract address of the trading pair
    #[serde(rename = "pairAddress")]
    pub pair_address: String,
    /// Optional labels associated with the pair (e.g., "v3", "stable")
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    /// Information about the base token in the pair
    #[serde(rename = "baseToken")]
    pub base_token: BaseToken,
    /// Information about the quote token in the pair
    #[serde(rename = "quoteToken")]
    pub quote_token: BaseToken,
    /// Price of base token in terms of quote token
    #[serde(rename = "priceNative")]
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub price_native: f64,
    /// Price of base token in USD
    #[serde(rename = "priceUsd")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_string_or_number")]
    pub price_usd: Option<f64>,
    /// Transaction statistics
    #[serde(rename = "txns")]
    pub transactions: PairTransactionCounts,
    /// Volume statistics in USD
    pub volume: VolumeChangePeriods,
    /// Price change percentages
    #[serde(rename = "priceChange")]
    pub price_change: PriceChangePeriods,
    /// Liquidity information
    #[serde(default)]
    pub liquidity: Option<Liquidity>,
    /// Fully diluted valuation in USD
    #[serde(default)]
    pub fdv: Option<f64>,
    /// Market capitalization in USD
    #[serde(rename = "marketCap")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_optional_string_or_number")]
    pub market_cap: Option<f64>,
    /// When the trading pair was created
    #[serde(rename = "pairCreatedAt")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_timestamp_to_datetime")]
    pub pair_created_at: Option<DateTime<Utc>>,
}

/// Response for API endpoints that return a single token pair.
///
/// This structure is used for responses like get_pair_by_token_address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinglePairResponse {
    /// The token pair data
    pub pair: TokenPair,
}

/// Response for API endpoints that return multiple token pairs.
///
/// This structure is used for responses like get_pairs_by_chain_and_address
/// and get_pairs_by_token_addresses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairResponse {
    /// List of token pairs
    pub pairs: Vec<TokenPair>,
}

/// Response for the search API endpoint.
///
/// This structure contains the search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// List of token pairs matching the search query
    pub pairs: Vec<TokenPair>,
}
