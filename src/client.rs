use crate::errors::{DexScreenerError, ErrorResponse};
use crate::models::{PairResponse, SearchResponse};
use crate::API_BASE_URL;
use reqwest::Client;
use serde::Deserialize;

/// Client for interacting with the DexScreener API.
///
/// This struct provides methods for making requests to the various endpoints
/// of the DexScreener API. It handles the construction of URLs, sending requests,
/// and parsing responses.
///
/// # Examples
///
/// ```no_run
/// use dexscreener_rs::DexScreenerClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = DexScreenerClient::new();
///
///     // Fetch data for a specific pair
///     let pair_data = client.get_pairs_by_chain_and_address(
///         "ethereum",
///         "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
///     ).await?;
///
///     println!("Found {} pair(s)", pair_data.pairs.len());
///
///     Ok(())
/// }
/// ```
pub struct DexScreenerClient {
    /// Base URL for the API
    base_url: String,
    /// HTTP client for making requests
    client: Client,
}

impl DexScreenerClient {
    /// Creates a new DexScreener API client with default configuration.
    ///
    /// This uses the standard DexScreener API base URL and default reqwest client settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use dexscreener_rs::DexScreenerClient;
    ///
    /// let client = DexScreenerClient::new();
    /// ```
    pub fn new() -> Self {
        DexScreenerClient {
            base_url: API_BASE_URL.to_string(),
            client: Client::new(),
        }
    }

    /// Creates a new DexScreener API client with a custom base URL.
    ///
    /// This is useful for testing or when the API URL changes.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL to use for API requests
    ///
    /// # Examples
    ///
    /// ```
    /// use dexscreener_rs::DexScreenerClient;
    ///
    /// let client = DexScreenerClient::with_base_url("https://api-test.dexscreener.com");
    /// ```
    pub fn with_base_url<S: Into<String>>(base_url: S) -> Self {
        DexScreenerClient {
            base_url: base_url.into(),
            client: Client::new(),
        }
    }

    /// Gets information about one or multiple pairs by chain ID and pair address.
    ///
    /// This method fetches detailed information about trading pairs on the specified blockchain.
    /// It's subject to a rate limit of 300 requests per minute.
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain identifier (e.g., "ethereum", "bsc", "polygon")
    /// * `pair_address` - The address of the trading pair contract
    ///
    /// # Returns
    ///
    /// A `Result` containing a `PairResponse` with the pair information if successful,
    /// or a `DexScreenerError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dexscreener_rs::DexScreenerClient;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DexScreenerClient::new();
    /// let response = client.get_pairs_by_chain_and_address(
    ///     "ethereum",
    ///     "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pairs_by_chain_and_address(
        &self,
        chain_id: &str,
        pair_address: &str,
    ) -> Result<PairResponse, DexScreenerError> {
        let url = format!(
            "{}/latest/dex/pairs/{}/{}",
            self.base_url, chain_id, pair_address
        );
        self.get_request(&url).await
    }

    /// Gets all pairs that include a specific token.
    ///
    /// This method retrieves all trading pairs containing the specified token address
    /// on the given blockchain. The API has a rate limit of 300 requests per minute.
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain identifier (e.g., "ethereum", "bsc", "polygon")
    /// * `token_address` - The address of the token
    ///
    /// # Returns
    ///
    /// A `Result` containing a `PairResponse` with all pairs including the token if successful,
    /// or a `DexScreenerError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dexscreener_rs::DexScreenerClient;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DexScreenerClient::new();
    /// // Get all pairs containing WETH on Ethereum
    /// let response = client.get_pair_by_token_address(
    ///     "ethereum",
    ///     "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pair_by_token_address(
        &self,
        chain_id: &str,
        token_address: &str,
    ) -> Result<PairResponse, DexScreenerError> {
        let url = format!(
            "{}/token-pairs/v1/{}/{}",
            self.base_url, chain_id, token_address
        );

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            // API returns an array of TokenPair objects
            let pairs: Vec<crate::models::TokenPair> = response.json().await?;
            Ok(PairResponse { pairs })
        } else {
            let error_response = response.json::<ErrorResponse>().await?;
            Err(DexScreenerError::ApiError(error_response))
        }
    }

    /// Gets pairs containing any of the specified token addresses.
    ///
    /// This method allows retrieving pairs for multiple tokens at once.
    /// The API limits this to a maximum of 30 token addresses per request.
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain identifier (e.g., "ethereum", "bsc", "polygon")
    /// * `token_addresses` - A vector of token addresses (maximum 30)
    ///
    /// # Returns
    ///
    /// A `Result` containing a `PairResponse` with all pairs containing any of the tokens,
    /// or a `DexScreenerError` if the request fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if more than 30 token addresses are provided.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dexscreener_rs::DexScreenerClient;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DexScreenerClient::new();
    /// let addresses = vec![
    ///     "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", // WETH
    ///     "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"  // USDC
    /// ];
    /// let response = client.get_pairs_by_token_addresses("ethereum", addresses).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_pairs_by_token_addresses(
        &self,
        chain_id: &str,
        token_addresses: Vec<&str>,
    ) -> Result<PairResponse, DexScreenerError> {
        if token_addresses.len() > 30 {
            return Err(DexScreenerError::new(
                "Too many token addresses. Maximum allowed is 30.",
            ));
        }

        let addresses_str = token_addresses.join(",");
        let url = format!("{}/tokens/v1/{}/{}", self.base_url, chain_id, addresses_str);

        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            // API returns an array of TokenPair objects
            let pairs: Vec<crate::models::TokenPair> = response.json().await?;
            Ok(PairResponse { pairs })
        } else {
            let error_response = response.json::<ErrorResponse>().await?;
            Err(DexScreenerError::ApiError(error_response))
        }
    }

    /// Searches for trading pairs matching a query.
    ///
    /// This endpoint allows searching for pairs by token name, symbol, or address.
    /// It has a rate limit of 300 requests per minute.
    ///
    /// # Arguments
    ///
    /// * `query` - The search query (token name, symbol, or address)
    ///
    /// # Returns
    ///
    /// A `Result` containing a `SearchResponse` with matching pairs if successful,
    /// or a `DexScreenerError` if the request fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dexscreener_rs::DexScreenerClient;
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DexScreenerClient::new();
    /// let response = client.search_pairs("ETH").await?;
    /// println!("Found {} pairs matching the query", response.pairs.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_pairs(&self, query: &str) -> Result<SearchResponse, DexScreenerError> {
        let url = format!("{}/latest/dex/search?q={}", self.base_url, query);
        self.get_request(&url).await
    }

    /// Internal method to make a GET request and parse the response.
    ///
    /// # Arguments
    ///
    /// * `url` - The full URL to request
    ///
    /// # Returns
    ///
    /// A `Result` containing the deserialized response if successful,
    /// or a `DexScreenerError` if the request fails.
    async fn get_request<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
    ) -> Result<T, DexScreenerError> {
        let response = self.client.get(url).send().await?;

        if response.status().is_success() {
            let response_data = response.json::<T>().await?;
            Ok(response_data)
        } else {
            let error_response = response.json::<ErrorResponse>().await?;
            Err(DexScreenerError::ApiError(error_response))
        }
    }
}

impl Default for DexScreenerClient {
    /// Creates a new client with default settings.
    ///
    /// This is equivalent to calling `DexScreenerClient::new()`.
    fn default() -> Self {
        Self::new()
    }
}
