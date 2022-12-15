use warp_completion_metadata::Signature;

#[cfg(not(feature = "json-embed"))]
pub mod commands;

#[cfg(feature = "json-embed")]
mod embed;

#[cfg(feature = "json-embed")]
pub fn signatures() -> Vec<Signature> {
    embed::signatures()
}

#[cfg(not(feature = "json-embed"))]
pub fn signatures() -> Vec<Signature> {
    commands::signatures()
}

#[cfg(not(feature = "json-embed"))]
pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    let name = name.as_ref();
    commands::signatures().iter().find(|sig| {
        sig.name == name
    }).cloned()
}

#[cfg(feature = "json-embed")]
pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    embed::signature_by_name(name)
}
