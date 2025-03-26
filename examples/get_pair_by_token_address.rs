use dexscreener_rs::DexScreenerClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create DexScreener client
    let client = DexScreenerClient::new();

    // Define chain ID and token address
    let chain_id = "solana";
    let token_address = "So11111111111111111111111111111111111111112"; // SOL

    println!("Fetching pairs for token on {}:", chain_id);
    println!("- {}", token_address);

    let response = client
        .get_pair_by_token_address(chain_id, token_address)
        .await?;

    // Check if any results were returned
    if response.pairs.is_empty() {
        println!("\nNo pairs found for the specified token");
        return Ok(());
    }

    println!("\nFound {} pairs for token:", response.pairs.len());

    // Iterate through all returned pairs and display information
    for (i, pair) in response.pairs.iter().enumerate() {
        println!("\n--- Pair {} ---", i + 1);
        println!("DEX: {}", pair.dex_id);
        println!("Pair Address: {}", pair.pair_address);
        println!(
            "Base Token: {} ({})",
            pair.base_token.name, pair.base_token.symbol
        );
        println!(
            "Quote Token: {} ({})",
            pair.quote_token.name, pair.quote_token.symbol
        );

        if let Some(price_usd) = pair.price_usd {
            println!("Price: ${:.6}", price_usd);
        } else {
            println!("Price: Not available in USD");
        }

        println!("Native Price: {}", pair.price_native);
        println!("24h Volume: ${:.2}", pair.volume.h24);

        // Display liquidity information (if available)
        if let Some(liquidity) = &pair.liquidity {
            if let Some(usd) = liquidity.usd {
                println!("Liquidity: ${:.2}", usd);
            }
        }

        // Display market cap and fully diluted valuation (if available)
        if let Some(market_cap) = pair.market_cap {
            println!("Market Cap: ${:.2}", market_cap);
        }

        if let Some(fdv) = pair.fdv {
            println!("Fully Diluted Valuation: ${:.2}", fdv);
        }

        // Transaction activity
        println!(
            "24h Transactions: {} buys, {} sells",
            pair.transactions.h24.buys, pair.transactions.h24.sells
        );

        println!("URL: {}", pair.url);

        // Price change
        println!("Price Change 24h: {:.2}%", pair.price_change.h24);

        // Creation time information (if available)
        if let Some(created_at) = pair.pair_created_at {
            println!("Pair Created At: {}", created_at);
        }

        // Add separator between pairs if there are multiple
        if i < response.pairs.len() - 1 {
            println!("\n-----------------------");
        }
    }

    Ok(())
}
