use dexscreener_rs::DexScreenerClient;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create DexScreener client
    let client = DexScreenerClient::new();

    // WETH-USDC Uniswap V3 pair on Ethereum
    let chain_id = "ethereum";
    let pair_address = "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640";

    println!(
        "Fetching information for {}/{} trading pair...",
        chain_id, pair_address
    );

    // Call API to get pair information
    let response = client
        .get_pairs_by_chain_and_address(chain_id, pair_address)
        .await?;

    // Check if any results were returned
    if response.pairs.is_empty() {
        println!("No pair information found");
        return Ok(());
    }

    // Get the first pair's information
    let pair = &response.pairs[0];

    // Print detailed pair information
    println!("\n--- Pair Information ---");
    println!("Chain: {}", pair.chain_id);
    println!("DEX: {}", pair.dex_id);
    println!("Pair Address: {}", pair.pair_address);
    println!("URL: {}", pair.url);

    // Token information
    println!("\n--- Token Information ---");
    println!(
        "Base Token: {} ({})",
        pair.base_token.name, pair.base_token.symbol
    );
    println!("Base Token Address: {}", pair.base_token.address);
    println!(
        "Quote Token: {} ({})",
        pair.quote_token.name, pair.quote_token.symbol
    );
    println!("Quote Token Address: {}", pair.quote_token.address);

    // Price and volume information
    println!("\n--- Price & Volume ---");
    println!("Native Price: {}", pair.price_native);

    if let Some(price_usd) = pair.price_usd {
        println!("USD Price: ${:.6}", price_usd);
    } else {
        println!("USD Price: Not available");
    }

    println!("\nPrice Change:");
    println!("  5 minutes: {:.2}%", pair.price_change.m5);
    println!("  1 hour: {:.2}%", pair.price_change.h1);
    println!("  6 hours: {:.2}%", pair.price_change.h6);
    println!("  24 hours: {:.2}%", pair.price_change.h24);

    println!("\nVolume USD:");
    println!("  5 minutes: ${:.2}", pair.volume.m5);
    println!("  1 hour: ${:.2}", pair.volume.h1);
    println!("  6 hours: ${:.2}", pair.volume.h6);
    println!("  24 hours: ${:.2}", pair.volume.h24);

    // Transaction statistics
    println!("\n--- Transaction Counts ---");
    println!(
        "Last 5 minutes: {} buys, {} sells",
        pair.transactions.m5.buys, pair.transactions.m5.sells
    );
    println!(
        "Last hour: {} buys, {} sells",
        pair.transactions.h1.buys, pair.transactions.h1.sells
    );
    println!(
        "Last 6 hours: {} buys, {} sells",
        pair.transactions.h6.buys, pair.transactions.h6.sells
    );
    println!(
        "Last 24 hours: {} buys, {} sells",
        pair.transactions.h24.buys, pair.transactions.h24.sells
    );

    // Liquidity information
    if let Some(liquidity) = &pair.liquidity {
        println!("\n--- Liquidity ---");
        if let Some(usd) = liquidity.usd {
            println!("USD: ${:.2}", usd);
        }
        println!("Base: {:.8} {}", liquidity.base, pair.base_token.symbol);
        println!("Quote: {:.8} {}", liquidity.quote, pair.quote_token.symbol);
    }

    // Other information
    if let Some(fdv) = pair.fdv {
        println!("\nFully Diluted Valuation: ${:.2}", fdv);
    }

    if let Some(market_cap) = pair.market_cap {
        println!("Market Cap: ${:.2}", market_cap);
    }

    if let Some(created_at) = pair.pair_created_at {
        println!("Pair Created At: {}", created_at);
    }

    Ok(())
}
