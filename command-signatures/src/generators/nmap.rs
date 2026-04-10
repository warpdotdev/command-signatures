use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub(super) fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("nmap")
        .add_generator("network_interfaces", network_interfaces_generator())
        .add_generator("nse_scripts", nse_scripts_generator())
}

/// Returns a cross-platform generator that lists network interface names.
///
/// Uses `/sys/class/net` on Linux, `ifconfig -l` on macOS, and falls back to
/// parsing `ifconfig` output.
fn network_interfaces_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "sh -c 'if [ -d /sys/class/net ]; then ls /sys/class/net; elif ifconfig -l >/dev/null 2>&1; then ifconfig -l | tr \" \" \"\\n\"; else ifconfig 2>/dev/null | grep -oE \"^[a-zA-Z0-9]+\" | sort -u; fi'",
        ),
        |output| {
            output
                .trim()
                .lines()
                .filter(|line| !line.is_empty())
                .map(|name| Suggestion::with_description(name.trim(), "Network interface"))
                .collect_unordered_results()
        },
    )
}

/// Returns a generator that lists available NSE (Nmap Scripting Engine) scripts
/// and script categories.
///
/// Searches common nmap data directories across Linux and macOS (Homebrew) for
/// `.nse` script files, and also provides built-in script categories.
fn nse_scripts_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "sh -c 'for d in /usr/share/nmap/scripts /usr/local/share/nmap/scripts /opt/homebrew/share/nmap/scripts; do if [ -d \"$d\" ]; then ls \"$d\"/*.nse 2>/dev/null | xargs -n1 basename | sed \"s/\\.nse$//\"; break; fi; done; printf \"all\\nauth\\nbroadcast\\nbrute\\ndefault\\ndiscovery\\ndos\\nexploit\\nexternal\\nfuzzer\\nintrusive\\nmalware\\nsafe\\nversion\\nvuln\\n\"'",
        ),
        |output| {
            let categories = [
                "all", "auth", "broadcast", "brute", "default", "discovery", "dos", "exploit",
                "external", "fuzzer", "intrusive", "malware", "safe", "version", "vuln",
            ];

            output
                .trim()
                .lines()
                .filter(|line| !line.is_empty())
                .map(|name| {
                    let trimmed = name.trim();
                    if categories.contains(&trimmed) {
                        Suggestion::with_description(trimmed, "Script category")
                    } else {
                        Suggestion::with_description(trimmed, "NSE script")
                    }
                })
                .collect_unordered_results()
        },
    )
}
