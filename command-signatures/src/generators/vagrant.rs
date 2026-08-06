use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Suggestion,
};

/// Lists the machine directories Vagrant keeps for the project enclosing the working
/// directory.
///
/// Walking up for `.vagrant/machines` is how Vagrant itself locates the project root, and
/// reading that directory avoids paying `vagrant status`'s multi-second Ruby start-up on
/// every keystroke. The second level of the listing is the provider each machine was
/// brought up with, which [`parse_machines`] turns into the suggestion's description.
const MACHINES_COMMAND: &str = "sh -c 'dir=$PWD; while [ -n \"$dir\" ]; do if [ -d \"$dir/.vagrant/machines\" ]; then cd \"$dir/.vagrant/machines\" && find . -mindepth 1 -maxdepth 2 -type d; break; fi; dir=${dir%/*}; done'";

/// Lists the box directories under the Vagrant home, honoring `$VAGRANT_HOME`.
const BOXES_COMMAND: &str = "sh -c 'ls -1 ${VAGRANT_HOME:-$HOME/.vagrant.d}/boxes'";

/// Parses the two-level `.vagrant/machines` listing into machine-name suggestions,
/// described by the provider directory nested under each machine when there is one.
pub(super) fn parse_machines(output: &str) -> GeneratorResults {
    let mut machines: Vec<(&str, Option<&str>)> = Vec::new();
    for entry in output
        .lines()
        .filter_map(|line| line.trim().strip_prefix("./"))
        .filter(|entry| !entry.is_empty())
    {
        match entry.split_once('/') {
            None => {
                if !machines.iter().any(|(name, _)| *name == entry) {
                    machines.push((entry, None));
                }
            }
            Some((name, provider)) => {
                if let Some((_, existing)) = machines.iter_mut().find(|(m, _)| *m == name) {
                    existing.get_or_insert(provider);
                }
            }
        }
    }
    machines
        .into_iter()
        .map(|(name, provider)| match provider {
            Some(provider) => Suggestion::with_description(name, provider),
            None => Suggestion::new(name),
        })
        .collect_unordered_results()
}

/// Parses the box directory listing into box-name suggestions.
///
/// Vagrant escapes the characters that cannot appear in a directory name when it stores a
/// box, so the placeholders have to be reversed to recover the name the CLI accepts. The
/// colon is un-escaped before the slash, mirroring the order Vagrant's own
/// `BoxCollection#undir_name` uses.
pub(super) fn parse_boxes(output: &str) -> GeneratorResults {
    output
        .lines()
        .map(str::trim)
        .filter(|directory| !directory.is_empty())
        .map(|directory| {
            let name = directory
                .replace("-VAGRANTCOLON-", ":")
                .replace("-VAGRANTSLASH-", "/");
            Suggestion::with_description(name, "Installed box")
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("vagrant")
        .add_generator(
            "vagrant_machines",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr(MACHINES_COMMAND),
                parse_machines,
            ),
        )
        .add_generator(
            "vagrant_boxes",
            Generator::script(
                CommandBuilder::single_command_and_ignore_stderr(BOXES_COMMAND),
                parse_boxes,
            ),
        )
}
