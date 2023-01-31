use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(not(feature = "json-embed"))]
    warp_completion_metadata::rust_generator::generate_rust_completion_specs()?;

    Ok(())
}
