use clap::Parser;
use dotenv::dotenv;
use ethers::prelude::*;
use std::env;
use anyhow::Result;

/// Simple CLI for checking an ETH balance.
#[derive(Parser)]
struct Cli {
    /// The Ethereum address to query
    #[arg(short, long)]
    address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load .env
    dotenv().ok();

    // 2. Read the RPC URL from env
    let rpc_url = env::var("ETH_RPC_URL")
        .expect("Set ETH_RPC_URL in .env");

    // 3. Parse CLI arguments
    let args = Cli::parse();
    let address: Address = args.address.parse()
        .expect("Invalid Ethereum address format");

    // 4. Create an HTTP provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // 5. Fetch balance (in Wei)
    let balance_wei = provider.get_balance(address, None).await?;

    // 6. Convert Wei → Ether (18 decimals)
    let balance_eth = ethers::utils::format_units(balance_wei, 18)?;

    // 7. Print it out
    println!("Balance of {address} is {balance_eth} ETH");

    Ok(())
}
