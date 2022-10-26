use itertools::Itertools;
use rust_embed::RustEmbed;
use warp_completion_metadata::Signature;

#[derive(RustEmbed)]
#[folder = "json"]
struct Assets;

pub(crate) fn signatures() -> Vec<Signature> {
    Assets::iter()
        .map(|path| Assets::get(&path))
        .filter_map(|embedded_file| {
            let embedded_data = embedded_file?.data;
            let json_content = std::str::from_utf8(&embedded_data).ok()?;
            let fig_command: warp_completion_metadata::fig_types::Command =
                serde_json::from_str(json_content).ok()?;
            Some(Signature::from(fig_command))
        })
        .collect_vec()
}