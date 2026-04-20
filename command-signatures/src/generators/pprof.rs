use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("pprof").add_generator(
        "saved_profiles",
        // Lists profile files saved in the default PPROF_TMPDIR ($HOME/pprof).
        // This is where pprof stores profiles fetched over HTTP by default and
        // is the most common location users keep captured profiles.
        Generator::script(
            CommandBuilder::single_command(
                "sh -c 'dir=\"${PPROF_TMPDIR:-$HOME/pprof}\"; [ -d \"$dir\" ] || exit 0; find \"$dir\" -maxdepth 2 -type f \\( -name \"*.pb.gz\" -o -name \"*.pprof\" -o -name \"*.prof\" -o -name \"profile\" -o -name \"profile.pb\" \\) 2>/dev/null'",
            ),
            |output| {
                output
                    .trim()
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|path| Suggestion::with_description(path.trim(), "Saved profile"))
                    .collect_unordered_results()
            },
        ),
    )
}
