use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use super::common::{pacman_installed_packages_generator, pacman_pkg_tar_files_in_cwd_generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("paru")
        .add_generator(
            "list_installed_packages",
            pacman_installed_packages_generator(),
        )
        .add_generator(
            // `paru -P/--show -c/--complete` prints AUR and repo package names, one per line,
            // for use by shell completion scripts. This is how paru's own official fish
            // completions list packages for `-S`/`sync`, unlike plain pacman, which only knows
            // about the sync repositories and not the AUR.
            "list_all_packages",
            Generator::script(CommandBuilder::single_command("paru -Pc"), |output| {
                let mut targets = Vec::new();
                for package_name in output.lines() {
                    targets.push(Suggestion::with_description(
                        package_name.to_string(),
                        "package",
                    ));
                }
                targets.into_iter().collect_unordered_results()
            }),
        )
        .add_generator(
            "list_all_pkg_tar_files_in_cwd",
            pacman_pkg_tar_files_in_cwd_generator(),
        )
}
