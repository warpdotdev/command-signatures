use crate::fig_types::Command;
use crate::Signature;
use anyhow::Result;
use convert_case::{Case, Casing};
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

/// Generates completions specs as rust files from the json stored in /json. Each command signature
/// is stored within its own mod within the commands module. Additionally, a function called
/// `signatures` is generated on the commands mod that returns a vector of all the `Signature`s that
/// were created.    
pub fn generate_rust_completion_specs() -> Result<()> {
    println!("cargo:rerun-if-changed=json");
    println!("cargo:rerun-if-changed=../../completion-metadata/src");

    // Create a commands module that will contain all of the signatures.
    std::fs::create_dir_all("src/commands")?;
    let parent_module = std::fs::File::create("src/commands/mod.rs")?;

    let mut signatures_added = Vec::new();

    // TODO(alokedesai): Support nested completions that are imported via Fig's `loadSpec` field.
    for entry in WalkDir::new("json").max_depth(1) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let file = File::open(entry.path())?;

            let mmap = unsafe { memmap::Mmap::map(&file) }?;
            let json_content = std::str::from_utf8(&mmap)?;

            let fig_command: crate::fig_types::Command = serde_json::from_str(json_content)?;

            let file_name = entry
                .file_name()
                .to_str()
                .expect("OsStr should convert to str")
                .replace(".json", "")
                .to_case(Case::Snake);

            // Create a module for each signature within the parent commands file.

            signatures_added.push(file_name.clone());

            write_completion(fig_command, file_name.as_str())?;

            writeln!(&parent_module, "pub mod {};", file_name)?;
        }
    }

    write_signatures_function(parent_module, &mut signatures_added)?;

    Ok(())
}

/// Writes a `signatures` function into the module at path `parent_modules`. The generated function
/// will look approximated like:
/// ```ignore
/// use warp_completion_metadata::*;
/// pub fn signatures() -> Vec<Signature> {
///     vec![
///         foo::signature(),
///         bar::signature(),
///         bazz:signature(),
///     ]
/// }
/// ```
fn write_signatures_function(
    mut parent_module: File,
    signatures_added: &mut Vec<String>,
) -> Result<()> {
    writeln!(parent_module, "\nuse warp_completion_metadata::*;")?;
    writeln!(parent_module, "pub fn signatures() -> Vec<Signature> {{")?;
    writeln!(parent_module, "vec![")?;
    for signature in signatures_added {
        writeln!(parent_module, "{}::signature(),", signature)?;
    }
    writeln!(parent_module, "]")?;
    writeln!(parent_module, "}}")?;

    Ok(())
}

/// Writes a signature into the path at `src/commands/<file_name>.rs`. The generated Rust will look
/// like:
/// ```ignore
/// use warp_completion_metadata::*;
/// pub fn signature -> Signature {
///    Signature {
///      ...
///    }
/// }
/// ```  
fn write_completion(fig_command: Command, file_name: &str) -> Result<()> {
    let module = std::fs::File::create(&format!("src/commands/{}.rs", file_name))?;

    writeln!(&module, "use warp_completion_metadata::*;")?;
    writeln!(&module, "\n#[allow(clippy::invisible_characters)]")?;
    writeln!(&module, r#"pub fn signature() -> Signature {{"#)?;
    writeln!(
        &module,
        "{}",
        uneval::to_string(Signature::from(fig_command))?
    )?;
    writeln!(&module, "}}")?;

    Ok(())
}
