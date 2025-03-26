use dexscreener_rs::DexScreenerClient;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create DexScreener client
    let client = DexScreenerClient::new();

    // Define chain ID and token addresses
    let chain_id = "solana";
    let token_addresses = vec![
        "So11111111111111111111111111111111111111112",  // SOL
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", // USDC
    ];

    println!("Fetching pairs for tokens on {}:", chain_id);
    for address in &token_addresses {
        println!("- {}", address);
    }

    // Call API to get trading pair information for these tokens
    let response = client
        .get_pairs_by_token_addresses(chain_id, token_addresses)
        .await?;

    // Check if any results were returned
    if response.pairs.is_empty() {
        println!("\nNo pairs found for the specified tokens");
        return Ok(());
    }

    println!("\nFound {} pairs:", response.pairs.len());

    // We might receive many pairs, so group them by DEX first
    let mut pairs_by_dex = HashMap::new();

    for pair in &response.pairs {
        pairs_by_dex
            .entry(pair.dex_id.clone())
            .or_insert_with(Vec::new)
            .push(pair);
    }

    // Print DEX group information
    println!("\n--- Pairs by DEX ---");
    for (dex, pairs) in &pairs_by_dex {
        println!("\n{} ({} pairs):", dex, pairs.len());

        // Print the first 3 pairs for each DEX
        for (i, pair) in pairs.iter().take(3).enumerate() {
            println!(
                "  {}. {} ({}) / {} ({})",
                i + 1,
                pair.base_token.symbol,
                pair.base_token.address,
                pair.quote_token.symbol,
                pair.quote_token.address
            );

            if let Some(price_usd) = pair.price_usd {
                println!("     Price: ${:.6}", price_usd);
            }
            println!("     24h Volume: ${:.2}", pair.volume.h24);
            println!("     URL: {}", pair.url);
        }

        // If there are more pairs, indicate to the user
        if pairs.len() > 3 {
            println!("     ... and {} more pairs", pairs.len() - 3);
        }
    }

    // Sort pairs by volume to find the most popular ones
    let mut sorted_pairs = response.pairs;
    sorted_pairs.sort_by(|a, b| {
        b.volume
            .h24
            .partial_cmp(&a.volume.h24)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("\n--- Top Pairs by 24h Volume ---");
    for (i, pair) in sorted_pairs.iter().take(5).enumerate() {
        println!(
            "\n{}. {} / {} ({})",
            i + 1,
            pair.base_token.symbol,
            pair.quote_token.symbol,
            pair.dex_id
        );

        if let Some(price_usd) = pair.price_usd {
            println!("   Price: ${:.6}", price_usd);
        }
        println!("   24h Volume: ${:.2}", pair.volume.h24);
        println!("   24h Change: {:.2}%", pair.price_change.h24);
        println!("   URL: {}", pair.url);
    }

    Ok(())
}
