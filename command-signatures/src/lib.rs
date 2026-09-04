#[cfg(feature = "embed-signatures")]
mod assets;
mod generators;
pub mod overrides;
pub mod powershell_autogenerator;

pub use generators::dynamic_command_signature_data;

#[cfg(feature = "embed-signatures")]
use assets::Assets;
pub use warp_completion_metadata::*;

#[cfg(feature = "embed-signatures")]
pub fn signature_by_name(name: impl AsRef<str>) -> Option<Signature> {
    let file_path = format!("{}.json", name.as_ref());
    Assets::get(&file_path).and_then(|embedded_file| {
        let json_content = std::str::from_utf8(&embedded_file.data).ok()?;
        let fig_command: warp_completion_metadata::fig_types::Command =
            serde_json::from_str(json_content).ok()?;
        let signatures: Vec<Signature> = fig_command.into();
        debug_assert!(
            signatures.len() <= 1,
            "Tried to fetch a signature by name for a signature that has multiple names"
        );
        signatures.into_iter().next()
    })
}

/// On web, we don't embed command signatures into the binary. All requests for a command signature return
/// None. In the future, we would like to investigate lazy loading this data.
#[cfg(not(feature = "embed-signatures"))]
pub fn signature_by_name(_name: impl AsRef<str>) -> Option<Signature> {
    None
}

#[cfg(feature = "embed-signatures")]
pub fn commands() -> Vec<Signature> {
    use itertools::Itertools;
    use rayon::prelude::*;

    Assets::iter()
        .collect_vec()
        .into_par_iter()
        .map(|path| Assets::get(&path))
        .filter_map(|embedded_file| {
            let embedded_data = embedded_file?.data;
            let json_content = std::str::from_utf8(&embedded_data).ok()?;
            let fig_command: warp_completion_metadata::fig_types::Command =
                serde_json::from_str(json_content).ok()?;
            Some(Vec::from(fig_command))
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::assets::all_signature_names;

    use super::*;

    fn get_generator_names_from_argument(arg: &Argument) -> Vec<&str> {
        let mut names = vec![];
        for arg_type in &arg.argument_types {
            if let ArgumentType::Generator(GeneratorName(name)) = arg_type {
                names.push(name.as_str());
            }
        }
        names
    }

    fn get_generator_names_from_option(opt: &Opt) -> Vec<&str> {
        opt.arguments()
            .iter()
            .flat_map(get_generator_names_from_argument)
            .collect_vec()
    }

    fn get_generator_names_from_signature(signature: &Signature) -> Vec<(&str, &str)> {
        std::iter::repeat(signature.name.as_str())
            .zip(
                // Combine generator names from arguments...
                signature
                    .arguments()
                    .iter()
                    .flat_map(get_generator_names_from_argument)
                    // generator names from options...
                    .chain(
                        signature
                            .options()
                            .iter()
                            .flat_map(get_generator_names_from_option),
                    )
                    // and generator names from subcommands.
                    .chain(
                        signature
                            .subcommands()
                            .iter()
                            .flat_map(get_generator_names_from_signature)
                            .map(|(_signature_name, generator_name)| generator_name),
                    ),
            )
            .collect_vec()
    }

    /// Verify that all generators referenced by command signatures are actually defined.
    #[test]
    fn all_referenced_generators_exist() {
        let generators = generators::dynamic_command_signature_data();
        let generator_names = generators
            .values()
            .flat_map(|dynamic_data| dynamic_data.generators().keys().map(|g| g.0.as_str()))
            .collect::<HashSet<_>>();
        assert!(
            !generator_names.is_empty(),
            "The bundled command signatures should reference at least one generator"
        );
        for signature in commands() {
            for (signature_name, generator_name) in get_generator_names_from_signature(&signature) {
                assert!(generator_names.contains(generator_name), "Did not find generator with name {generator_name} (from signature {signature_name})");
            }
        }
    }

    #[test]
    fn all_referenced_alias_generators_exist() {
        let generators = generators::dynamic_command_signature_data();
        let alias_generator_names = generators
            .values()
            .flat_map(|dynamic_data| dynamic_data.aliases().keys().map(|g| g.0.as_str()))
            .collect::<HashSet<_>>();
        assert!(
            !alias_generator_names.is_empty(),
            "The bundled command signatures should reference at least one alias generator"
        );
        for signature in commands() {
            if let Some(alias_generator_name) = signature.alias_generator {
                assert!(
                    alias_generator_names.contains(alias_generator_name.0.as_str()),
                    "Did not find generator with name {alias_generator_name} (from signature {})",
                    signature.name
                );
            }
        }
    }

    /// Verify that all command signatures are well-formed JSON and valid for our deserialization
    /// schema.
    #[test]
    fn all_command_specs_succeed_deserialization() {
        for name in all_signature_names() {
            signature_by_name(name).unwrap_or_else(|| panic!("{} failed to deserialize", name));
        }
    }

    /// Ensures no unquoted '\n' can be found.
    fn has_unsafe_newlines(str: &str) -> bool {
        let mut quote_char: Option<char> = None;
        let chars = str.chars().peekable();
        let mut is_escaped = false;

        for c in chars {
            match c {
                '\'' | '"' => {
                    if !is_escaped {
                        if quote_char.is_none() {
                            quote_char = Some(c);
                        } else if quote_char == Some(c) {
                            quote_char = None;
                        }
                    }
                }
                '\n' => {
                    if quote_char.is_none() && !is_escaped {
                        return true;
                    }
                }
                _ => {}
            }
            if c == '\\' {
                is_escaped = !is_escaped;
            } else {
                is_escaped = false;
            }
        }

        false
    }

    #[test]
    fn test_has_unsafe_newlines() {
        assert!(!has_unsafe_newlines("echo 'ahoy\nworld'"));
        assert!(has_unsafe_newlines("echo \\'bon voyage\nworld'"));
        assert!(!has_unsafe_newlines("echo \\\\'bon voyage\nworld'"));

        assert!(!has_unsafe_newlines("echo \"ciao\nworld\""));
        assert!(has_unsafe_newlines("echo \\\"danke\nworld\""));
        assert!(!has_unsafe_newlines("echo \\\\\"ello\nworld\""));

        assert!(!has_unsafe_newlines("echo \"fred's\nworld\""));
        assert!(!has_unsafe_newlines("echo 'george says \"\nworld\"'"));

        assert!(!has_unsafe_newlines("echo hello\\nworld"));
        assert!(has_unsafe_newlines("echo imagine\nworld"));
    }

    #[test]
    /// We want to send commands through TMUX control mode, and our current implementation
    /// only supports one-line commands. This may be a constraint we don't need to
    /// uphold in the future.
    fn all_command_specs_have_no_newlines() {
        let generators = generators::dynamic_command_signature_data();

        let token_test_cases = ["true", "hello world", "1", "1.0", "127.0.0.1", "\\n"];

        for (generator_name, completion_data) in generators {
            completion_data
                .generators()
                .values()
                .for_each(|generator| match &generator.process {
                    GeneratorProcess::CommandFromTokens(func) => {
                        token_test_cases.iter().for_each(|&tokens| {
                            let builder = func(&[tokens, " "], true, &[]);
                            let trailing_whitespace_result = builder.build(Shell::Posix);
                            assert!(
                                !has_unsafe_newlines(&trailing_whitespace_result),
                                "[has_trailing_whitespace: true] Tokens: `{}` - Generator `{}` has an unquoted newline in it: `{}`",
                                tokens,
                                generator_name,
                                trailing_whitespace_result
                            );
                            let command_builder = func(&[tokens], false, &[]);
                            let no_trailing_whitespace_result = command_builder.build(Shell::Posix);
                            assert!(
                                !has_unsafe_newlines(&no_trailing_whitespace_result),
                                "[has_trailing_whitespace: false] Tokens: `{}` - Generator `{}` has an unquoted newline in it: `{}`",
                                tokens,
                                generator_name,
                                no_trailing_whitespace_result
                            );
                        });
                    }
                    GeneratorProcess::ShellCommand(str) => {
                        let str = str.build(Shell::Posix);
                        assert!(
                            !has_unsafe_newlines(&str),
                            "Generator `{}` has an unquoted newline in it: `{}`",
                            generator_name,
                            str
                        );
                    }
                });
        }
    }
}

/// Invariant: no handwritten command spec has two distinct option entries directly declaring
/// the same short flag (https://github.com/warpdotdev/command-signatures/issues/400).
///
/// A hard "no new conflicts" gate is backed by an exact, temporary baseline allowlist of the 45
/// conflicts that predate this rule (tracked by
/// https://github.com/warpdotdev/command-signatures/issues/402). The baseline is compared as
/// complete records &mdash; `(file, command path, short flag)` plus the ordered claimant list of
/// source indices and raw `name` arrays &mdash; so adding a claimant, removing one, reordering
/// options, or swapping a claimant is review-visible even when the key itself is unchanged.
#[cfg(test)]
mod duplicate_short_flags {
    use crate::fig_types::Command;
    use crate::validation::{find_short_flag_conflicts, ShortFlagClaimant, ShortFlagConflict};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// `(file, command path, short flag)`.
    type ConflictKey = (String, Vec<String>, String);
    /// Ordered `(zero-based source index, complete raw name array)` per claimant.
    type Claimants = Vec<(usize, Vec<String>)>;

    struct BaselineConflict {
        file: &'static str,
        command_path: &'static [&'static str],
        flag: &'static str,
        claimants: &'static [(usize, &'static [&'static str])],
    }

    /// Temporary allowlist of pre-existing conflicts. See the module doc comment above.
    ///
    /// Do not add entries to this baseline for newly introduced conflicts: fix the command spec
    /// instead. Removing an entry because its conflict was fixed is always welcome; see
    /// https://github.com/warpdotdev/command-signatures/issues/402.
    const SHORT_FLAG_CONFLICT_BASELINE: &[BaselineConflict] = &[
        BaselineConflict {
            file: "brew.json",
            command_path: &["brew", "services"],
            flag: "-h",
            claimants: &[(3, &["-h", "--help"]), (7, &["-h", "--help"])],
        },
        BaselineConflict {
            file: "brew.json",
            command_path: &["brew", "services"],
            flag: "-v",
            claimants: &[(2, &["-v", "--verbose"]), (6, &["-v", "--verbose"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "package"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "deploy"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "delete"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "status"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "logs"],
            flag: "-n",
            claimants: &[(2, &["--name", "-n"]), (7, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "exec"],
            flag: "-n",
            claimants: &[(3, &["--name", "-n"]), (4, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "pause"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "svc", "resume"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "job", "package"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "job", "deploy"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "job", "delete"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "task", "exec"],
            flag: "-n",
            claimants: &[(3, &["--name", "-n"]), (4, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "task", "delete"],
            flag: "-n",
            claimants: &[(2, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "deploy"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "package"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "deploy"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "delete"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "status"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "logs"],
            flag: "-n",
            claimants: &[(2, &["--name", "-n"]), (7, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "exec"],
            flag: "-n",
            claimants: &[(3, &["--name", "-n"]), (4, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "pause"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "svc", "resume"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "job", "package"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "job", "deploy"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "job", "delete"],
            flag: "-n",
            claimants: &[(1, &["--name", "-n"]), (2, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "task", "exec"],
            flag: "-n",
            claimants: &[(3, &["--name", "-n"]), (4, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "copilot.json",
            command_path: &["copilot", "help", "task", "delete"],
            flag: "-n",
            claimants: &[(2, &["--name", "-n"]), (3, &["--name", "-n"])],
        },
        BaselineConflict {
            file: "flutter.json",
            command_path: &["flutter", "assemble"],
            flag: "-d",
            claimants: &[(2, &["-d", "--device-id"]), (4, &["-d", "--define"])],
        },
        BaselineConflict {
            file: "flutter.json",
            command_path: &["flutter", "symbolize"],
            flag: "-d",
            claimants: &[(2, &["-d", "--device-id"]), (4, &["-d", "--debug-info"])],
        },
        BaselineConflict {
            file: "kubecolor.json",
            command_path: &["kubecolor", "expose"],
            flag: "-l",
            claimants: &[(7, &["-l", "--selector"]), (12, &["-l", "--labels"])],
        },
        BaselineConflict {
            file: "kubectl.json",
            command_path: &["kubectl", "expose"],
            flag: "-l",
            claimants: &[(7, &["-l", "--selector"]), (12, &["-l", "--labels"])],
        },
        BaselineConflict {
            file: "oc.json",
            command_path: &["oc", "expose"],
            flag: "-l",
            claimants: &[(7, &["-l", "--selector"]), (12, &["-l", "--labels"])],
        },
        BaselineConflict {
            file: "pscale.json",
            command_path: &["pscale", "completion"],
            flag: "-h",
            claimants: &[(1, &["--help", "-h"]), (2, &["--help", "-h"])],
        },
        BaselineConflict {
            file: "pscale.json",
            command_path: &["pscale", "connect"],
            flag: "-h",
            claimants: &[(0, &["--help", "-h"]), (8, &["--help", "-h"])],
        },
        BaselineConflict {
            file: "pulumi.json",
            command_path: &["pulumi", "import"],
            flag: "-f",
            claimants: &[(17, &["-f", "--skip-preview"]), (19, &["-f", "--file"])],
        },
        BaselineConflict {
            file: "rsync.json",
            command_path: &["rsync"],
            flag: "-h",
            claimants: &[(111, &["-h", "--human-readable"]), (134, &["-h", "--help"])],
        },
        BaselineConflict {
            file: "sfdx.json",
            command_path: &["sfdx", "auth:accesstoken:store"],
            flag: "-s",
            claimants: &[
                (3, &["-s", "--setdefaultdevhubusername"]),
                (4, &["-s", "--setdefaultusername"]),
            ],
        },
        BaselineConflict {
            file: "stripe.json",
            command_path: &["stripe", "events resend"],
            flag: "-v",
            claimants: &[
                (11, &["-v", "--stripe-version"]),
                (18, &["-v", "--version"]),
            ],
        },
        BaselineConflict {
            file: "stripe.json",
            command_path: &["stripe", "get"],
            flag: "-v",
            claimants: &[
                (11, &["-v", "--stripe-version"]),
                (18, &["-v", "--version"]),
            ],
        },
        BaselineConflict {
            file: "stripe.json",
            command_path: &["stripe", "post"],
            flag: "-v",
            claimants: &[(8, &["-v", "--stripe-version"]), (15, &["-v", "--version"])],
        },
        BaselineConflict {
            file: "stripe.json",
            command_path: &["stripe", "delete"],
            flag: "-v",
            claimants: &[(8, &["-v", "--stripe-version"]), (15, &["-v", "--version"])],
        },
        BaselineConflict {
            file: "yarn.json",
            command_path: &["yarn"],
            flag: "-s",
            claimants: &[(35, &["-s", "--silent"]), (39, &["-s", "--silent"])],
        },
        BaselineConflict {
            file: "zapier.json",
            command_path: &["zapier-platform-cli", "scaffold"],
            flag: "-d",
            claimants: &[(0, &["-d", "--dest"]), (5, &["-d", "--debug"])],
        },
    ];

    fn baseline_map() -> BTreeMap<ConflictKey, Claimants> {
        let mut map = BTreeMap::new();
        for entry in SHORT_FLAG_CONFLICT_BASELINE {
            let key: ConflictKey = (
                format!("command-signatures/json/{}", entry.file),
                entry.command_path.iter().map(|s| s.to_string()).collect(),
                entry.flag.to_string(),
            );
            let claimants: Claimants = entry
                .claimants
                .iter()
                .map(|(index, names)| (*index, names.iter().map(|s| s.to_string()).collect()))
                .collect();
            assert!(
                map.insert(key, claimants).is_none(),
                "duplicate baseline entry for {} {:?} {}",
                entry.file,
                entry.command_path,
                entry.flag
            );
        }
        map
    }

    /// Only the top-level handwritten `command-signatures/json/*.json` files, excluding
    /// `autogenerated/` and `overrides/`.
    fn handwritten_json_files() -> Vec<PathBuf> {
        let json_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("json");
        let mut files: Vec<PathBuf> = fs::read_dir(&json_dir)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", json_dir.display()))
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        files.sort();
        files
    }

    fn collect_actual_conflicts() -> BTreeMap<ConflictKey, Claimants> {
        let mut actual = BTreeMap::new();
        for path in handwritten_json_files() {
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default()
                .to_string();
            let relative_file = format!("command-signatures/json/{filename}");
            let content = fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
            let command: Command = serde_json::from_str(&content)
                .unwrap_or_else(|err| panic!("{relative_file} failed to deserialize: {err}"));

            for conflict in find_short_flag_conflicts(&command) {
                let key: ConflictKey = (
                    relative_file.clone(),
                    conflict.command_path.clone(),
                    conflict.flag.clone(),
                );
                let claimants: Claimants = conflict
                    .claimants
                    .iter()
                    .map(|claimant| (claimant.index, claimant.names.clone()))
                    .collect();
                assert!(
                    actual.insert(key, claimants).is_none(),
                    "validator produced two records for the same conflict key in {relative_file}"
                );
            }
        }
        actual
    }

    /// Builds the same `ShortFlagConflict` the validator itself produces from a `(key,
    /// claimants)` record, so baseline-diff diagnostics go through the shared
    /// `ShortFlagConflict::describe` formatter (canonical space-delimited path, quoted flag, raw
    /// JSON name arrays, one-based positions) instead of a bespoke, zero-based debug rendering.
    fn describe_record(key: &ConflictKey, claimants: &Claimants) -> String {
        let (file, command_path, flag) = key;
        let conflict = ShortFlagConflict {
            command_path: command_path.clone(),
            flag: flag.clone(),
            claimants: claimants
                .iter()
                .map(|(index, names)| ShortFlagClaimant {
                    index: *index,
                    names: names.clone(),
                })
                .collect(),
        };
        conflict.describe(file)
    }

    /// Compares collected conflicts against the baseline as complete records. Returns one
    /// human-readable failure message per problem found, each rendered with the shared
    /// `ShortFlagConflict::describe` diagnostic format:
    /// - a collected conflict whose key is absent from the baseline (a new conflict);
    /// - a collected conflict whose ordered claimant list differs from the baseline's (the
    ///   claimant identity changed even though the key is unchanged);
    /// - a baseline entry with no matching collected conflict (a stale entry that must be
    ///   removed).
    fn diff_against_baseline(
        actual: &BTreeMap<ConflictKey, Claimants>,
        baseline: &BTreeMap<ConflictKey, Claimants>,
    ) -> Vec<String> {
        let mut failures = Vec::new();

        for (key, claimants) in actual {
            match baseline.get(key) {
                None => failures.push(format!(
                    "new duplicate short flag conflict not in baseline:\n  {}",
                    describe_record(key, claimants)
                )),
                Some(expected_claimants) if expected_claimants != claimants => failures.push(format!(
                    "claimant identity changed for baselined conflict:\n  expected: {}\n  actual:   {}",
                    describe_record(key, expected_claimants),
                    describe_record(key, claimants)
                )),
                _ => {}
            }
        }

        for (key, expected_claimants) in baseline {
            if !actual.contains_key(key) {
                failures.push(format!(
                    "stale baseline entry, remove it:\n  {}",
                    describe_record(key, expected_claimants)
                ));
            }
        }

        failures
    }

    #[test]
    fn duplicate_short_flags_match_baseline() {
        let actual = collect_actual_conflicts();
        let baseline = baseline_map();
        let failures = diff_against_baseline(&actual, &baseline);
        assert!(
            failures.is_empty(),
            "\n{}\n\nSee https://github.com/warpdotdev/command-signatures/issues/400 and the \
            temporary baseline allowlist tracked by \
            https://github.com/warpdotdev/command-signatures/issues/402.",
            failures.join("\n")
        );
    }

    #[test]
    fn only_top_level_handwritten_json_is_discovered() {
        let files = handwritten_json_files();
        assert!(
            !files.is_empty(),
            "expected to discover handwritten JSON files"
        );
        for path in &files {
            let path_str = path.to_string_lossy();
            assert!(
                !path_str.contains("autogenerated"),
                "discovered file should not come from autogenerated/: {path_str}"
            );
            assert!(
                !path_str.contains("overrides"),
                "discovered file should not come from overrides/: {path_str}"
            );
        }
        assert!(
            files
                .iter()
                .any(|path| path.file_name().and_then(|n| n.to_str()) == Some("flutter.json")),
            "expected to discover the top-level flutter.json fixture"
        );
    }

    /// Regression test: an otherwise matching baselined key that gains a third claimant must
    /// fail, even though `(file, command path, short flag)` is unchanged.
    #[test]
    fn diff_detects_a_claimant_added_to_a_baselined_conflict() {
        let key: ConflictKey = (
            "tool.json".to_string(),
            vec!["tool".to_string()],
            "-t".to_string(),
        );
        let mut baseline = BTreeMap::new();
        baseline.insert(
            key.clone(),
            vec![(0, vec!["-t".to_string()]), (1, vec!["-t".to_string()])],
        );

        let mut actual = BTreeMap::new();
        actual.insert(
            key,
            vec![
                (0, vec!["-t".to_string()]),
                (1, vec!["-t".to_string()]),
                (2, vec!["-t".to_string()]),
            ],
        );

        let failures = diff_against_baseline(&actual, &baseline);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("claimant identity changed"));
    }

    /// Regression test: replacing one claimant of an otherwise matching baselined key must fail,
    /// even though `(file, command path, short flag)` is unchanged.
    #[test]
    fn diff_detects_a_claimant_replaced_in_a_baselined_conflict() {
        let key: ConflictKey = (
            "tool.json".to_string(),
            vec!["tool".to_string()],
            "-t".to_string(),
        );
        let mut baseline = BTreeMap::new();
        baseline.insert(
            key.clone(),
            vec![
                (0, vec!["-t".to_string(), "--type-a".to_string()]),
                (1, vec!["-t".to_string(), "--type-b".to_string()]),
            ],
        );

        let mut actual = BTreeMap::new();
        actual.insert(
            key,
            vec![
                (0, vec!["-t".to_string(), "--type-a".to_string()]),
                (1, vec!["-t".to_string(), "--type-c".to_string()]),
            ],
        );

        let failures = diff_against_baseline(&actual, &baseline);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("claimant identity changed"));
    }

    #[test]
    fn diff_flags_new_conflicts_not_in_baseline() {
        let baseline = BTreeMap::new();
        let mut actual = BTreeMap::new();
        actual.insert(
            (
                "tool.json".to_string(),
                vec!["tool".to_string()],
                "-t".to_string(),
            ),
            vec![(0, vec!["-t".to_string()]), (1, vec!["-t".to_string()])],
        );

        let failures = diff_against_baseline(&actual, &baseline);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("new duplicate short flag conflict"));
    }

    #[test]
    fn diff_flags_stale_baseline_entries() {
        let mut baseline = BTreeMap::new();
        baseline.insert(
            (
                "tool.json".to_string(),
                vec!["tool".to_string()],
                "-t".to_string(),
            ),
            vec![(0, vec!["-t".to_string()]), (1, vec!["-t".to_string()])],
        );
        let actual = BTreeMap::new();

        let failures = diff_against_baseline(&actual, &baseline);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("stale baseline entry"));
    }

    /// Pins the exact, one-based, source-actionable diagnostic text (matching
    /// `ShortFlagConflict::describe`) for all three failure modes, so a future change cannot
    /// silently regress to a bare zero-based debug rendering of the claimant records.
    #[test]
    fn diff_messages_use_the_shared_one_based_diagnostic_format() {
        let key: ConflictKey = (
            "tool.json".to_string(),
            vec!["tool".to_string()],
            "-t".to_string(),
        );
        let expected_text = "tool.json: command \"tool\": duplicate short flag \"-t\" is used by options #1 [\"-t\", \"--type-a\"] and #2 [\"-t\", \"--type-b\"]";
        let changed_text = "tool.json: command \"tool\": duplicate short flag \"-t\" is used by options #1 [\"-t\", \"--type-a\"] and #2 [\"-t\", \"--type-c\"]";
        let baselined_claimants = vec![
            (0, vec!["-t".to_string(), "--type-a".to_string()]),
            (1, vec!["-t".to_string(), "--type-b".to_string()]),
        ];
        let changed_claimants = vec![
            (0, vec!["-t".to_string(), "--type-a".to_string()]),
            (1, vec!["-t".to_string(), "--type-c".to_string()]),
        ];

        // New conflict: not present in the baseline at all.
        let mut actual = BTreeMap::new();
        actual.insert(key.clone(), baselined_claimants.clone());
        let failures = diff_against_baseline(&actual, &BTreeMap::new());
        assert_eq!(failures.len(), 1);
        assert!(
            failures[0].contains(expected_text),
            "expected {:?} to contain {:?}",
            failures[0],
            expected_text
        );

        // Claimant identity changed: both the expected and actual one-based diagnostics appear.
        let mut baseline = BTreeMap::new();
        baseline.insert(key.clone(), baselined_claimants.clone());
        let mut actual = BTreeMap::new();
        actual.insert(key.clone(), changed_claimants);
        let failures = diff_against_baseline(&actual, &baseline);
        assert_eq!(failures.len(), 1);
        assert!(
            failures[0].contains(expected_text),
            "expected {:?} to contain {:?}",
            failures[0],
            expected_text
        );
        assert!(
            failures[0].contains(changed_text),
            "expected {:?} to contain {:?}",
            failures[0],
            changed_text
        );

        // Stale baseline entry: no longer produced, but still described with the shared format.
        let mut baseline = BTreeMap::new();
        baseline.insert(key, baselined_claimants);
        let failures = diff_against_baseline(&BTreeMap::new(), &baseline);
        assert_eq!(failures.len(), 1);
        assert!(
            failures[0].contains(expected_text),
            "expected {:?} to contain {:?}",
            failures[0],
            expected_text
        );
    }
}
