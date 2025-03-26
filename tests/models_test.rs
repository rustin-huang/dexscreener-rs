#[cfg(test)]
mod tests {
    use chrono::{DateTime, TimeZone, Utc};
    use dexscreener_rs::models::*;
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_deserialize_string_or_number() {
        #[derive(Debug, Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_string_or_number")]
            value: f64,
        }

        // Test with numeric value
        let json_numeric = r#"{"value": 123.45}"#;
        let result: TestStruct = serde_json::from_str(json_numeric).unwrap();
        assert_eq!(result.value, 123.45);

        // Test with string value
        let json_string = r#"{"value": "678.90"}"#;
        let result: TestStruct = serde_json::from_str(json_string).unwrap();
        assert_eq!(result.value, 678.90);
    }

    #[test]
    fn test_deserialize_optional_string_or_number() {
        #[derive(Debug, Deserialize)]
        struct TestStruct {
            #[serde(default)]
            #[serde(deserialize_with = "deserialize_optional_string_or_number")]
            value: Option<f64>,
        }

        // Test with numeric value
        let json_numeric = r#"{"value": 123.45}"#;
        let result: TestStruct = serde_json::from_str(json_numeric).unwrap();
        assert_eq!(result.value, Some(123.45));

        // Test with string value
        let json_string = r#"{"value": "678.90"}"#;
        let result: TestStruct = serde_json::from_str(json_string).unwrap();
        assert_eq!(result.value, Some(678.90));

        // Test with empty string (should be None)
        let json_empty = r#"{"value": ""}"#;
        let result: TestStruct = serde_json::from_str(json_empty).unwrap();
        assert_eq!(result.value, None);

        // Test with missing field
        let json_missing = r#"{}"#;
        let result: TestStruct = serde_json::from_str(json_missing).unwrap();
        assert_eq!(result.value, None);
    }

    #[test]
    fn test_deserialize_timestamp_to_datetime() {
        #[derive(Debug, Deserialize)]
        struct TestStruct {
            #[serde(deserialize_with = "deserialize_timestamp_to_datetime")]
            timestamp: Option<DateTime<Utc>>,
        }

        // Test with Unix timestamp (milliseconds)
        let json_timestamp = r#"{"timestamp": 1620250931000}"#;
        let result: TestStruct = serde_json::from_str(json_timestamp).unwrap();
        let expected = Utc.timestamp_opt(1620250931, 0).unwrap();
        assert_eq!(result.timestamp.unwrap().timestamp(), expected.timestamp());

        // Test with RFC3339 string
        let json_rfc3339 = r#"{"timestamp": "2021-05-05T12:22:11Z"}"#;
        let result: TestStruct = serde_json::from_str(json_rfc3339).unwrap();
        let expected = Utc.with_ymd_and_hms(2021, 5, 5, 12, 22, 11).unwrap();
        assert_eq!(result.timestamp.unwrap(), expected);

        // Test with timestamp as string
        let json_timestamp_string = r#"{"timestamp": "1620250931000"}"#;
        let result: TestStruct = serde_json::from_str(json_timestamp_string).unwrap();
        let expected = Utc.timestamp_opt(1620250931, 0).unwrap();
        assert_eq!(result.timestamp.unwrap().timestamp(), expected.timestamp());

        // Test with empty string (should be None)
        let json_empty = r#"{"timestamp": ""}"#;
        let result: TestStruct = serde_json::from_str(json_empty).unwrap();
        assert_eq!(result.timestamp, None);
    }

    #[test]
    fn test_base_token_deserialization() {
        let json = r#"{
            "address": "0xabc123",
            "name": "Ethereum",
            "symbol": "ETH"
        }"#;

        let token: BaseToken = serde_json::from_str(json).unwrap();
        assert_eq!(token.address, "0xabc123");
        assert_eq!(token.name, "Ethereum");
        assert_eq!(token.symbol, "ETH");
    }

    #[test]
    fn test_transaction_count_deserialization() {
        let json = r#"{
            "buys": 123,
            "sells": 456
        }"#;

        let txn_count: TransactionCount = serde_json::from_str(json).unwrap();
        assert_eq!(txn_count.buys, 123);
        assert_eq!(txn_count.sells, 456);
    }

    #[test]
    fn test_liquidity_deserialization() {
        // Test with all fields present
        let json = r#"{
            "usd": 1000000.5,
            "base": 100.25,
            "quote": 500000.75
        }"#;

        let liquidity: Liquidity = serde_json::from_str(json).unwrap();
        assert_eq!(liquidity.usd, Some(1000000.5));
        assert_eq!(liquidity.base, 100.25);
        assert_eq!(liquidity.quote, 500000.75);

        // Test with usd as string
        let json_string_usd = r#"{
            "usd": "2000000.5",
            "base": 200.5,
            "quote": 1000000.25
        }"#;

        let liquidity: Liquidity = serde_json::from_str(json_string_usd).unwrap();
        assert_eq!(liquidity.usd, Some(2000000.5));
        assert_eq!(liquidity.base, 200.5);
        assert_eq!(liquidity.quote, 1000000.25);

        // Test with missing usd
        let json_missing_usd = r#"{
            "base": 300.75,
            "quote": 1500000.5
        }"#;

        let liquidity: Liquidity = serde_json::from_str(json_missing_usd).unwrap();
        assert_eq!(liquidity.usd, None);
        assert_eq!(liquidity.base, 300.75);
        assert_eq!(liquidity.quote, 1500000.5);
    }

    #[test]
    fn test_time_periods_float_deserialization() {
        // Test with all numeric values
        let json = r#"{
            "m5": 10.5,
            "h1": 20.75,
            "h6": 100.25,
            "h24": 500.5
        }"#;

        let periods: TimePeriodsFloat = serde_json::from_str(json).unwrap();
        assert_eq!(periods.m5, 10.5);
        assert_eq!(periods.h1, 20.75);
        assert_eq!(periods.h6, 100.25);
        assert_eq!(periods.h24, 500.5);

        // Test with string values
        let json_strings = r#"{
            "m5": "15.25",
            "h1": "25.5",
            "h6": "105.75",
            "h24": "505.25"
        }"#;

        let periods: TimePeriodsFloat = serde_json::from_str(json_strings).unwrap();
        assert_eq!(periods.m5, 15.25);
        assert_eq!(periods.h1, 25.5);
        assert_eq!(periods.h6, 105.75);
        assert_eq!(periods.h24, 505.25);

        // Test with missing values (should use defaults)
        let json_missing = r#"{
            "h1": 30.25,
            "h24": 510.75
        }"#;

        let periods: TimePeriodsFloat = serde_json::from_str(json_missing).unwrap();
        assert_eq!(periods.m5, 0.0); // default
        assert_eq!(periods.h1, 30.25);
        assert_eq!(periods.h6, 0.0); // default
        assert_eq!(periods.h24, 510.75);
    }

    #[test]
    fn test_token_pair_deserialization() {
        let json = json!({
            "chainId": "ethereum",
            "dexId": "uniswap",
            "url": "https://info.uniswap.org/#/pairs/0x1234",
            "pairAddress": "0x1234",
            "labels": ["v3", "stable"],
            "baseToken": {
                "address": "0xabc",
                "name": "Ethereum",
                "symbol": "ETH"
            },
            "quoteToken": {
                "address": "0xdef",
                "name": "USD Coin",
                "symbol": "USDC"
            },
            "priceNative": "3000.5",
            "priceUsd": 3000.5,
            "txns": {
                "m5": { "buys": 10, "sells": 5 },
                "h1": { "buys": 60, "sells": 30 },
                "h6": { "buys": 360, "sells": 180 },
                "h24": { "buys": 1440, "sells": 720 }
            },
            "volume": {
                "m5": "1000.5",
                "h1": 6000.25,
                "h6": "36000.75",
                "h24": 144000.5
            },
            "priceChange": {
                "m5": 0.1,
                "h1": "1.0",
                "h6": 2.0,
                "h24": "5.0"
            },
            "liquidity": {
                "usd": "10000000.5",
                "base": 1000.25,
                "quote": "3000000.75"
            },
            "fdv": 5000000000.5,
            "marketCap": "2500000000.25",
            "pairCreatedAt": 1620250931000_i64
        });

        let pair: TokenPair = serde_json::from_value(json).unwrap();

        // Check basic fields
        assert_eq!(pair.chain_id, "ethereum");
        assert_eq!(pair.dex_id, "uniswap");
        assert_eq!(pair.pair_address, "0x1234");

        // Check tokens
        assert_eq!(pair.base_token.symbol, "ETH");
        assert_eq!(pair.quote_token.symbol, "USDC");

        // Check prices
        assert_eq!(pair.price_native, 3000.5);
        assert_eq!(pair.price_usd, Some(3000.5));

        // Check transactions
        assert_eq!(pair.transactions.h24.buys, 1440);
        assert_eq!(pair.transactions.h24.sells, 720);

        // Check volume and price change
        assert_eq!(pair.volume.h24, 144000.5);
        assert_eq!(pair.price_change.h24, 5.0);

        // Check liquidity
        assert!(pair.liquidity.is_some());
        if let Some(liq) = &pair.liquidity {
            assert_eq!(liq.usd, Some(10000000.5));
            assert_eq!(liq.base, 1000.25);
            assert_eq!(liq.quote, 3000000.75);
        }

        // Check other fields
        assert_eq!(pair.fdv, Some(5000000000.5));
        assert_eq!(pair.market_cap, Some(2500000000.25));

        // Check creation time
        assert!(pair.pair_created_at.is_some());
        let created_at = pair.pair_created_at.unwrap();
        assert_eq!(created_at.timestamp(), 1620250931);
    }

    #[test]
    fn test_pair_response_deserialization() {
        let json = r#"{
            "pairs": [
                {
                    "chainId": "ethereum",
                    "dexId": "uniswap",
                    "url": "https://info.uniswap.org/#/pairs/0x1234",
                    "pairAddress": "0x1234",
                    "baseToken": {
                        "address": "0xabc",
                        "name": "Ethereum",
                        "symbol": "ETH"
                    },
                    "quoteToken": {
                        "address": "0xdef",
                        "name": "USD Coin",
                        "symbol": "USDC"
                    },
                    "priceNative": 3000.5,
                    "priceUsd": 3000.5,
                    "txns": {
                        "m5": { "buys": 10, "sells": 5 },
                        "h1": { "buys": 60, "sells": 30 },
                        "h6": { "buys": 360, "sells": 180 },
                        "h24": { "buys": 1440, "sells": 720 }
                    },
                    "volume": {
                        "m5": 1000.5,
                        "h1": 6000.25,
                        "h6": 36000.75,
                        "h24": 144000.5
                    },
                    "priceChange": {
                        "m5": 0.1,
                        "h1": 1.0,
                        "h6": 2.0,
                        "h24": 5.0
                    }
                }
            ]
        }"#;

        let response: PairResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.pairs.len(), 1);

        let pair = &response.pairs[0];
        assert_eq!(pair.chain_id, "ethereum");
        assert_eq!(pair.base_token.symbol, "ETH");
        assert_eq!(pair.quote_token.symbol, "USDC");
    }

    #[test]
    fn test_search_response_deserialization() {
        let json = r#"{
            "pairs": [
                {
                    "chainId": "ethereum",
                    "dexId": "uniswap",
                    "url": "https://info.uniswap.org/#/pairs/0x1234",
                    "pairAddress": "0x1234",
                    "baseToken": {
                        "address": "0xabc",
                        "name": "Ethereum",
                        "symbol": "ETH"
                    },
                    "quoteToken": {
                        "address": "0xdef",
                        "name": "USD Coin",
                        "symbol": "USDC"
                    },
                    "priceNative": 3000.5,
                    "priceUsd": 3000.5,
                    "txns": {
                        "m5": { "buys": 10, "sells": 5 },
                        "h1": { "buys": 60, "sells": 30 },
                        "h6": { "buys": 360, "sells": 180 },
                        "h24": { "buys": 1440, "sells": 720 }
                    },
                    "volume": {
                        "m5": 1000.5,
                        "h1": 6000.25,
                        "h6": 36000.75,
                        "h24": 144000.5
                    },
                    "priceChange": {
                        "m5": 0.1,
                        "h1": 1.0,
                        "h6": 2.0,
                        "h24": 5.0
                    }
                }
            ]
        }"#;

        let response: SearchResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.pairs.len(), 1);

        let pair = &response.pairs[0];
        assert_eq!(pair.chain_id, "ethereum");
        assert_eq!(pair.dex_id, "uniswap");
    }

    #[test]
    fn test_token_pair_with_missing_optional_fields() {
        let json = json!({
            "chainId": "ethereum",
            "dexId": "uniswap",
            "url": "https://info.uniswap.org/#/pairs/0x1234",
            "pairAddress": "0x1234",
            "baseToken": {
                "address": "0xabc",
                "name": "Ethereum",
                "symbol": "ETH"
            },
            "quoteToken": {
                "address": "0xdef",
                "name": "USD Coin",
                "symbol": "USDC"
            },
            "priceNative": 3000.5,
            "txns": {
                "m5": { "buys": 10, "sells": 5 },
                "h1": { "buys": 60, "sells": 30 },
                "h6": { "buys": 360, "sells": 180 },
                "h24": { "buys": 1440, "sells": 720 }
            },
            "volume": {
                "m5": 1000.5,
                "h1": 6000.25,
                "h6": 36000.75,
                "h24": 144000.5
            },
            "priceChange": {
                "m5": 0.1,
                "h1": 1.0,
                "h6": 2.0,
                "h24": 5.0
            }
            // Missing priceUsd, liquidity, fdv, marketCap, pairCreatedAt
        });

        let pair: TokenPair = serde_json::from_value(json).unwrap();

        // Check basic fields
        assert_eq!(pair.chain_id, "ethereum");
        assert_eq!(pair.pair_address, "0x1234");

        // Check optional fields are None
        assert_eq!(pair.price_usd, None);
        assert_eq!(pair.liquidity, None);
        assert_eq!(pair.fdv, None);
        assert_eq!(pair.market_cap, None);
        assert_eq!(pair.pair_created_at, None);
    }
}
