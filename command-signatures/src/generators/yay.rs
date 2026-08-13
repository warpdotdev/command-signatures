/// `yay` is a pacman-compatible AUR helper: it wraps `pacman` for repository packages and adds
/// AUR support on top, using pacman's own CLI syntax (see `json/yay.json`, modeled on
/// `json/pacman.json`).
///
/// For enumerating *available* packages (`-S`/`--sync`), we can't just reuse pacman's
/// `pacman -Slq`, since that only lists repo packages and would reproduce the exact bug being
/// fixed here (no AUR completions). Instead we shell out to `yay -Pc` (`yay --show --complete`),
/// which yay's own man page describes as: "Print a list of all AUR and repo packages. This
/// allows shell completion and is not intended to be used directly by the user." It's also
/// exactly what yay's own bundled bash/zsh completion scripts use for this purpose. We prefer
/// it over querying the AUR RPC directly, since the AUR has well over 100k packages and
/// enumerating it live on every keystroke would be far too slow for interactive completion;
/// `-Pc` instead reads from yay's local completion cache, which it refreshes periodically (see
/// `--completioninterval`, default once a day) rather than on every invocation. The tradeoff is
/// that the very first call (before any cache exists) can be slow while that cache is built, but
/// every completion after that is fast.
///
/// For installed-package and local `.pkg.tar` file completions (`-Q`, `-R`, `-U`), we shell out
/// to `pacman` directly rather than `yay`, since those never touch the AUR: pacman's own
/// database is authoritative regardless of which frontend originally installed a package, and
/// there's no reason to pay yay's startup cost for data pacman already has.
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

use super::pacman::{
    list_installed_packages_generator, list_packages, list_pkg_tar_files_in_cwd_generator,
};

pub fn yay_generators() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("yay")
        .add_generator(
            "list_installed_packages",
            list_installed_packages_generator(),
        )
        .add_generator(
            "list_all_packages",
            Generator::script(CommandBuilder::single_command("yay -Pc"), list_packages),
        )
        .add_generator(
            "list_all_pkg_tar_files_in_cwd",
            list_pkg_tar_files_in_cwd_generator(),
        )
}
