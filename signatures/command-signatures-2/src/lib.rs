use warp_completion_metadata::Signature;

#[cfg(not(feature = "json-embed"))]
pub mod commands;

#[cfg(feature = "json-embed")]
mod embed;


#[cfg(feature = "json-embed")]
pub fn signatures() -> Vec<Signature> {
    return embed::signatures()
}

#[cfg(not(feature = "json-embed"))]
pub fn signatures() -> Vec<Signature> {
    commands::signatures()
}
