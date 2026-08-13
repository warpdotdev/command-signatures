/// `paru` is a pacman-compatible AUR helper: like `yay`, it wraps `pacman` for repository
/// packages and adds AUR support on top, using pacman's own CLI syntax (see `json/paru.json`,
/// modeled on `json/pacman.json`).
///
/// See `super::yay` for the full rationale, which applies identically here: `paru -Pc`
/// (`paru --show --complete`) is paru's own purpose-built completion command ("Print a list of
/// all AUR and repo packages. This allows shell completion and is not intended to be used
/// directly by the user."), used by paru's own bundled completion scripts, and backed by a
/// locally refreshed cache so it stays fast for interactive use despite the AUR having well over
/// 100k packages. Installed-package and local `.pkg.tar` file completions shell out to `pacman`
/// directly, since that data doesn't depend on paru at all.
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

use super::pacman::{
    list_installed_packages_generator, list_packages, list_pkg_tar_files_in_cwd_generator,
};

pub fn paru_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("paru")
        .add_generator(
            "list_installed_packages",
            list_installed_packages_generator(),
        )
        .add_generator(
            "list_all_packages",
            Generator::script(CommandBuilder::single_command("paru -Pc"), list_packages),
        )
        .add_generator(
            "list_all_pkg_tar_files_in_cwd",
            list_pkg_tar_files_in_cwd_generator(),
        )
}
