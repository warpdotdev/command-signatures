use anyhow::Result;

fn main() -> Result<()> {
    warp_completion_metadata::rust_generator::generate_rust_completion_specs()
}
