use alloy::{
    network::EthereumWallet,
    primitives::{address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};

sol!(
    #[sol(rpc)]
    IERC20,
    "src/abi.json"
);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let http_url = std::env::var("RPC_HTTP")?;
    let private_key: PrivateKeySigner = std::env::var("PRIVATE_KEY")?.parse()?;

    // For running on anvil. Anvil seems to have no issue with hanging on any of the watch functions.
    // let http_url = "http://localhost:8545";
    // let wallet_address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    // let private_key: PrivateKeySigner = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;

    let usdc_address = address!("af88d065e77c8cC2239327C5EDb3A432268e5831");
    let weth_address = address!("82aF49447D8a07e3bd95BD0d56f35241523fBab1");

    let wallet = EthereumWallet::from(private_key);
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(http_url.parse()?);

    let iweth = IERC20::new(weth_address, &provider);
    let iusdc = IERC20::new(usdc_address, &provider);

    // This should just be the uniswap router for arbitrum, pick what ever address to approve you like
    let approval_address = address!("4752ba5dbc23f44d87826276bf6fd6b1c372ad24");
    println!("trying first tx");
    let tx_hash = iweth
        .approve(approval_address, U256::MAX)
        .send()
        .await?
        // On windows it seems to hang execution here
        .watch()
        .await?;
    println!("{tx_hash:?}");
    println!("trying second tx");
    let tx_hash_2 = iusdc
        .approve(approval_address, U256::MAX)
        .send()
        .await?
        // On ubuntu execution seems to hang here
        .watch()
        .await?;
    println!("{tx_hash_2:?}");
    Ok(())
}
