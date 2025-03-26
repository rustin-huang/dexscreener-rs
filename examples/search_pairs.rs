use dexscreener_rs::DexScreenerClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create DexScreener client
    let client = DexScreenerClient::new();

    // Search query
    let query = "SOL/USDC";

    println!("Searching for pairs with query '{}'...", query);

    // Call API to search for trading pairs
    let response = client.search_pairs(query).await?;

    // Check if any results were returned
    if response.pairs.is_empty() {
        println!("No pairs found matching the query");
        return Ok(());
    }

    println!("\nFound {} pairs:", response.pairs.len());

    // Print search results
    for (i, pair) in response.pairs.iter().enumerate() {
        println!("\n--- Pair {} ---", i + 1);
        println!("Chain: {}", pair.chain_id);
        println!("DEX: {}", pair.dex_id);
        println!(
            "Pair: {} ({}) / {} ({})",
            pair.base_token.name,
            pair.base_token.symbol,
            pair.quote_token.name,
            pair.quote_token.symbol
        );

        if let Some(price_usd) = pair.price_usd {
            println!("Price: ${:.6}", price_usd);
        }

        println!("24h Volume: ${:.2}", pair.volume.h24);
        println!("24h Change: {:.2}%", pair.price_change.h24);
        println!("URL: {}", pair.url);
    }

    Ok(())
}
