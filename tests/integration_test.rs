use dexscreener_rs::DexScreenerClient;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_pair() {
        let client = DexScreenerClient::new();
        // Test with a well-known Ethereum pair (WETH-USDC on Uniswap V3)
        let pair_address = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640";

        let result = client
            .get_pairs_by_chain_and_address("ethereum", pair_address)
            .await;
        assert!(result.is_ok(), "API call failed: {:?}", result.err());

        let response = result.unwrap();
        assert!(!response.pairs.is_empty(), "No pairs returned");

        let pair = &response.pairs[0];
        assert_eq!(
            pair.pair_address.to_lowercase(),
            pair_address.to_lowercase()
        );
        assert_eq!(pair.chain_id.to_lowercase(), "ethereum");
        // WETH-USDC pair check
        assert!(
            (pair.base_token.symbol.to_uppercase() == "WETH"
                && pair.quote_token.symbol.to_uppercase() == "USDC")
                || (pair.base_token.symbol.to_uppercase() == "USDC"
                    && pair.quote_token.symbol.to_uppercase() == "WETH")
        );
    }

    #[tokio::test]
    async fn test_get_pairs_by_addresses() {
        let client = DexScreenerClient::new();
        // On Ethereum, get pairs for WETH and USDC
        let token_address = "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"; // WETH

        let result = client
            .get_pair_by_token_address("ethereum", token_address)
            .await;
        assert!(result.is_ok(), "API call failed: {:?}", result.err());

        let response = result.unwrap();
        assert!(!response.pairs.is_empty(), "No pairs returned");

        let pair = &response.pairs[0];
        assert!(
            pair.base_token.address.to_lowercase() == token_address.to_lowercase()
                || pair.quote_token.address.to_lowercase() == token_address.to_lowercase()
        );
        assert_eq!(pair.chain_id.to_lowercase(), "ethereum");
    }

    #[tokio::test]
    async fn test_search_pairs() {
        let client = DexScreenerClient::new();
        let query = "ETH";

        let result = client.search_pairs(query).await;
        assert!(result.is_ok(), "API call failed: {:?}", result.err());

        let response = result.unwrap();
        assert!(!response.pairs.is_empty(), "No pairs returned");
    }

    #[tokio::test]
    async fn test_error_handling() {
        let client = DexScreenerClient::new();
        // Use an invalid chain ID to test error handling
        let result = client
            .get_pairs_by_chain_and_address("invalid_chain", "0x1234")
            .await;

        // We expect this to fail due to invalid chain
        assert!(result.is_err(), "Expected an error for invalid chain ID");
    }

    #[tokio::test]
    async fn test_get_pairs_by_token_addresses() {
        let client = DexScreenerClient::new();
        let token_addresses = vec![
            "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2", // WETH
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48", // USDC
        ];

        let result = client
            .get_pairs_by_token_addresses("ethereum", token_addresses.clone())
            .await;
        assert!(result.is_ok(), "API call failed: {:?}", result.err());

        let response = result.unwrap();
        assert!(!response.pairs.is_empty(), "No pairs returned");

        // Check if any returned pair includes the requested tokens
        let has_requested_token = response.pairs.iter().any(|pair| {
            token_addresses.iter().any(|&addr| {
                addr.to_lowercase() == pair.base_token.address.to_lowercase()
                    || addr.to_lowercase() == pair.quote_token.address.to_lowercase()
            })
        });

        assert!(
            has_requested_token,
            "No pairs found containing the requested tokens"
        );
    }
}
