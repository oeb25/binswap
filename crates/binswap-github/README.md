[![Crates.io](https://img.shields.io/crates/v/binswap-github.svg)](https://crates.io/crates/binswap-github)
[![Workflow Status](https://github.com/oeb25/binswap/workflows/main/badge.svg)](https://github.com/oeb25/binswap/actions?query=workflow%3A%22main%22)

# binswap-github

Download and swap binaries from GitHub

## Usage

`binswap` uses the same infrastructure as
[`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) to
determine where the latest binaries are stored. `binswap-github` is the
backend to do this for GitHub specifically. It uses the GitHub releases to
download binaries for a supported target, and then downloads them to a
specified location, or optionally swaps them with the currently executed
binary.

This is particularly useful if you distribute binaries outside of package
managers or in environments where the users are not expected to have Rust
nor installed. With crate, you can bundle the updating mechanism into the
distributed binary.

## Example

The following example downloads the latest release [`ripgrep` from
GitHub](https://github.com/BurntSushi/ripgrep/releases), and swaps it with
the currently executed binary. `.dry_run(true)` is added here to simulate
the execution, but not perform the update.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    binswap_github::builder()
        .repo_author("BurntSushi")
        .repo_name("ripgrep")
        .asset_name("ripgrep")
        .bin_name("rg")
        .dry_run(true)
        .build()?
        .fetch_and_write_in_place_of_current_exec()
        .await?;

    Ok(())
}
```

The following does the same, but just writes the resulting binary to a new
file.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    binswap_github::builder()
        .repo_author("BurntSushi")
        .repo_name("ripgrep")
        .asset_name("ripgrep")
        .bin_name("rg")
        .dry_run(true)
        .build()?
        .fetch_and_write_to("./rg")
        .await?;

    Ok(())
}
```

## License

Licensed under either of

* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
