use color_eyre::Result;
use tracing_subscriber::prelude::*;

async fn run() -> Result<()> {
    binswap_github::builder()
        .repo_author("BurntSushi")
        .repo_name("ripgrep")
        .asset_name("ripgrep")
        .bin_name("rg")
        .check_with_cmd("--help")
        .build()?
        .fetch_and_write_in_place_of_current_exec()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::Registry::default()
        .with(tracing_error::ErrorLayer::default())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_timer(tracing_subscriber::fmt::time::uptime()))
        .with(tracing_subscriber::filter::FilterFn::new(|m| {
            !m.target().contains("hyper")
        }))
        .init();

    run().await?;

    Ok(())
}
