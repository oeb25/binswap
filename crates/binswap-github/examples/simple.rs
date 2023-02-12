#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    binswap_github::builder()
        .repo_author("BurntSushi")
        .repo_name("ripgrep")
        .asset_name("ripgrep")
        .bin_name("rg")
        .build()?
        .fetch_and_write_in_place_of_current_exec()
        .await?;

    Ok(())
}
