use warp_completion_metadata::CommandSignatureGenerators;

use crate::generators::git::worktrees_generator;

pub fn generator() -> CommandSignatureGenerators {
    // `cd` completes filesystem folders via its template; the worktrees
    // generator additionally surfaces the repo's git worktrees (identified by
    // path) so they can be jumped to without remembering their directory names.
    CommandSignatureGenerators::new("cd").add_generator("worktrees", worktrees_generator())
}
