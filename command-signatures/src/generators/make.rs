use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResults,
    GeneratorResultsCollector, Importance, Order, Priority, Suggestion,
};

lazy_static! {
    /// Regex to parse Makefiles for targets with either leading or inline comments that could be
    /// used as descriptions. The regex can be parsed as follows:
    /// `((?:^#.*\n)*)`: Capture group to match any leading comments (comments on the line before a
    ///   target).
    /// `(?:^\.[A-Z_]+:.*\n)`: non-capture group to avoid matching on some upper cased special
    ///   targets.
    /// `(^\S*?)`: Capture group to capture the target name.
    /// `(?:\s#+[ \t]*(.+))`: Match any trailing comments (while only capturing the name of the
    ///   comment) of the target. Such as `target: {command} # {trailing comment}`
    static ref MAKE_TARGET_RE: Regex =
        Regex::new(r"(?m)((?:^#.*\n)*)(?:^\.[A-Z_]+:.*\n)*(^\S*?):.*?(?:\s#+[ \t]*(.+))?$")
            .unwrap();
    static ref STARTS_WITH_COMMENT: Regex = Regex::new(r"^#+\s*").unwrap();
    static ref SPECIAL_TARGETS: HashSet<&'static str> = HashSet::from_iter([
        ".PHONY",
        ".SUFFIXES",
        ".DEFAULT",
        ".PRECIOUS",
        ".INTERMEDIATE",
        ".SECONDARY",
        ".SECONDEXPANSION",
        ".DELETE_ON_ERROR",
        ".IGNORE",
        ".LOW_RESOLUTION_TIME",
        ".SILENT",
        ".EXPORT_ALL_VARIABLES",
        ".NOTPARALLEL",
        ".ONESHELL",
        ".POSIX",
    ]);
}

fn list_targets_post_process(output: &str) -> GeneratorResults {
    MAKE_TARGET_RE
        .captures_iter(output)
        .filter_map(|capture| {
            let target = capture.get(2)?.as_str();
            if SPECIAL_TARGETS.contains(target) {
                return None;
            }

            let leading_comment = capture
                .get(1)
                .map(|capture_match| capture_match.as_str().trim());

            // The regex may not have a match for the capture group matching inline_comment. Both
            // target and leading comment should always have a match.
            let inline_comment = capture
                .get(3)
                .map(|capture_match| capture_match.as_str().trim());

            // Determine what the description should be based on the present of either a leading
            // comment or inline comment on the target. If neither exist, fallback to "Make target"
            // as the description
            let description = match (inline_comment, leading_comment) {
                (Some(inline_comment), _) if !inline_comment.is_empty() => inline_comment.into(),
                (_, Some(leading_comment)) if !leading_comment.is_empty() => {
                    // Remove the first part of the match so that we have the comment without any
                    // leading `#`s.
                    STARTS_WITH_COMMENT.replace(leading_comment, "")
                }
                // If there are no comments, fallback to `Make target` as the description.
                _ => "Make target".into(),
            };

            Some(
                Suggestion::with_description(target.trim(), description.trim())
                    .with_priority(Priority::Global(Importance::More(Order(80)))),
            )
        })
        .collect_unordered_results()
}

/// Shell command that prints the root Makefile plus every file it pulls in through `include`,
/// `-include`, and `sinclude` directives, so that targets split across included files are
/// surfaced (GNU make treats them as one unified ruleset).
///
/// A POSIX-sh `visit()` function walks the include graph: it `cat`s a file, then uses a small
/// `awk` program to extract that file's include paths and recurses into each. `awk` only extracts
/// the paths (it never opens them), and every path is passed to the shell as a quoted argument —
/// never interpolated into a command — so a malicious Makefile cannot inject commands. The awk
/// extractor only matches `include`/`-include`/`sinclude` directives indented with **spaces**, not
/// a leading tab: in make a tab-indented line is a recipe (a shell command), so a tab-indented
/// `include`-looking line is deliberately not followed. A trailing `# comment` on the directive is
/// stripped before the operands are split, so comment words are not mistaken for include paths.
///
/// Security boundary: completion runs against whatever repository the user has `cd`'d into, which
/// may be untrusted. Containment is enforced in two layers so a hostile Makefile cannot make
/// tab-completion read files outside the working tree before the user ever runs `make`:
///   1. `safe()` (lexical) skips absolute (`/etc/...`), home-relative (`~/...`), and `..`-escaping
///      include paths outright.
///   2. `realpath` (symlink-resolving) canonicalizes each remaining candidate and the `case` guard
///      follows it only when it stays under `$root` (`pwd -P`). This stops escapes a lexical check
///      cannot — e.g. a repo that ships `evil -> /etc` and does `include evil/passwd`, where the
///      path is lexically in-tree but resolves outside it.
///
/// GNU make resolves include paths relative to its working directory (not the including file), so a
/// legitimate project-local include is always a relative, non-`..` path resolving under the root
/// and is unaffected. A `seen` set of canonical paths guards against include cycles.
///
/// `realpath` is required for the symlink-resolving layer; if it is unavailable the command falls
/// back to `cat [Mm]akefile` (top-level targets only) rather than following includes unsafely.
///
/// Known limitation: include paths that rely on globbing (`include dir/*.mk`), make variables
/// (`include $(VAR)`), or that contain whitespace are not expanded/followed — resolving them
/// safely would require a shell (injection risk) or evaluating the Makefile. Absolute,
/// home-relative, `..`-escaping, and symlink-escaping includes are intentionally not followed (see
/// the security boundary above); their targets are simply not surfaced in completion.
const LIST_TARGETS_COMMAND: &str = r##"root=$(pwd -P)||exit 0;command -v realpath >/dev/null 2>&1||{ cat [Mm]akefile 2>/dev/null;exit 0;};seen="|";visit(){ rp=$(realpath -- "$1" 2>/dev/null)||return 0;case "$rp" in "$root"|"$root"/*) ;; *) return 0;; esac;case "$seen" in *"|$rp|"*) return 0;; esac;seen="$seen$rp|";cat -- "$rp" 2>/dev/null;set -f;for inc in $(awk 'function safe(p){return p!~/^\//&&p!~/^~/&&p!~/(^|\/)\.\.(\/|$)/} /^ *[-s]?include[ \t]+/{match($0,/^ *[-s]?include[ \t]+/);rest=substr($0,RLENGTH+1);sub(/#.*/,"",rest);n=split(rest,parts,/[ \t]+/);for(i=1;i<=n;i++)if(parts[i]!=""&&safe(parts[i]))print parts[i]}' "$rp" 2>/dev/null);do set +f;visit "$inc";set -f;done;set +f;};for f in [Mm]akefile;do [ -e "$f" ]&&visit "$f";done"##;

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("make").add_generator(
        "list_targets",
        Generator::script(
            CommandBuilder::single_command(LIST_TARGETS_COMMAND),
            list_targets_post_process,
        ),
    )
}

#[cfg(test)]
mod tests {
    use crate::generators::make::list_targets_post_process;

    #[test]
    fn test_list_targets_generator() {
        let output = r#"TARGET = alacritty

ASSETS_DIR = extra
RELEASE_DIR = target/release
MANPAGE = $(ASSETS_DIR)/alacritty.man
MANPAGE-MSG = $(ASSETS_DIR)/alacritty-msg.man
TERMINFO = $(ASSETS_DIR)/alacritty.info
COMPLETIONS_DIR = $(ASSETS_DIR)/completions
COMPLETIONS = $(COMPLETIONS_DIR)/_alacritty \
	$(COMPLETIONS_DIR)/alacritty.bash \
	$(COMPLETIONS_DIR)/alacritty.fish

APP_NAME = Alacritty.app
APP_TEMPLATE = $(ASSETS_DIR)/osx/$(APP_NAME)
APP_DIR = $(RELEASE_DIR)/osx
APP_BINARY = $(RELEASE_DIR)/$(TARGET)
APP_BINARY_DIR = $(APP_DIR)/$(APP_NAME)/Contents/MacOS
APP_EXTRAS_DIR = $(APP_DIR)/$(APP_NAME)/Contents/Resources
APP_COMPLETIONS_DIR = $(APP_EXTRAS_DIR)/completions

DMG_NAME = Alacritty.dmg
DMG_DIR = $(RELEASE_DIR)/osx

vpath $(TARGET) $(RELEASE_DIR)
vpath $(APP_NAME) $(APP_DIR)
vpath $(DMG_NAME) $(APP_DIR)

.PHONY: all

all: help

help: ## Print this help message
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

## Build a release binary
binary: $(TARGET)-native
binary-universal: $(TARGET)-universal ## Build a universal release binary

app: $(APP_NAME)-native ## Create an Alacritty.app
app-universal: $(APP_NAME)-universal

dmg: $(DMG_NAME)-native"#;

        let generator_results = list_targets_post_process(output);
        let name_and_description: Vec<_> = generator_results
            .suggestions
            .into_iter()
            .map(|suggestion| (suggestion.exact_string, suggestion.description))
            .collect();

        assert_eq!(
            name_and_description,
            [
                ("all", Some("Make target")),
                ("help", Some("Print this help message")),
                ("binary", Some("Build a release binary")),
                ("binary-universal", Some("Build a universal release binary")),
                ("app", Some("Create an Alacritty.app")),
                ("app-universal", Some("Make target")),
                ("dmg", Some("Make target")),
            ]
            .into_iter()
            .map(|(name, description)| (name.to_owned(), description.map(ToOwned::to_owned)))
            .collect::<Vec<_>>()
        );
    }

    /// Runs the real generator command via `sh -c` against a temp project and returns the
    /// resulting `(target, description)` pairs.
    fn run_list_targets_in(files: &[(&str, &str)]) -> Vec<(String, Option<String>)> {
        use super::LIST_TARGETS_COMMAND;
        use std::process::Command;
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::time::{SystemTime, UNIX_EPOCH};

        // Unique per call so the tests can run in parallel without sharing a temp dir.
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("cs_make_include_test_{nanos}_{seq}"));
        for (path, contents) in files {
            let full = dir.join(path);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, contents).unwrap();
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(LIST_TARGETS_COMMAND)
            .current_dir(&dir)
            .output()
            .expect("failed to run list_targets command");

        std::fs::remove_dir_all(&dir).ok();

        assert!(
            output.status.success(),
            "command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8(output.stdout).unwrap();
        list_targets_post_process(&stdout)
            .suggestions
            .into_iter()
            .map(|suggestion| (suggestion.exact_string, suggestion.description))
            .collect()
    }

    /// End-to-end test of the `list_targets` command: builds a temp project whose targets are
    /// split across `include`d files (one reached via a nested include, plus a missing optional
    /// `-include`), runs the actual generator command, and feeds its output through
    /// `list_targets_post_process`. Guards against the regression in #11705 where only top-level
    /// Makefile targets were suggested, and confirms `##` descriptions survive the include path.
    #[test]
    fn test_list_targets_command_follows_includes() {
        let results = run_list_targets_in(&[
            (
                // The trailing `# makefiles/leak.mk` is a Make comment, not a second operand: it
                // must be stripped before splitting so the comment word is not followed as an
                // include path (the file exists below precisely to catch that regression).
                "Makefile",
                "include makefiles/main.mk # makefiles/leak.mk\n-include does-not-exist.mk\n\nhelp: ## root help\n\t@echo root\n",
            ),
            (
                "makefiles/main.mk",
                "include makefiles/nested.mk\n\nup-main: ## start\n\tdocker compose up -d\ndown-main:\n\tdocker compose down\n",
            ),
            (
                "makefiles/nested.mk",
                "nested-target: ## from a nested include\n\t@echo nested\n",
            ),
            (
                "makefiles/leak.mk",
                "comment-leaked: ## must not surface\n\t@echo bad\n",
            ),
        ]);

        for expected in ["help", "up-main", "down-main", "nested-target"] {
            assert!(
                results.iter().any(|(target, _)| target == expected),
                "expected target `{expected}` reached through includes, got {results:?}"
            );
        }

        // The trailing-comment operand must not have been followed.
        assert!(
            !results.iter().any(|(target, _)| target == "comment-leaked"),
            "a word in a trailing `#` comment must not be followed as an include, got {results:?}"
        );

        // A `##` description defined in an included file must survive the include expansion.
        assert!(
            results
                .iter()
                .any(|(target, description)| target == "nested-target"
                    && description.as_deref() == Some("from a nested include")),
            "expected `nested-target` to keep its included `##` description, got {results:?}"
        );
    }

    /// An `include` directive pointing at a real directory is an invalid Makefile that `make`
    /// itself rejects. The generator resolves it (it is in-tree) but `cat`/`awk` on a directory
    /// produce no targets and no error reaches stdout, so the command must still exit cleanly and
    /// surface the top-level targets rather than breaking or returning nothing.
    #[test]
    fn test_list_targets_command_survives_directory_include() {
        let results = run_list_targets_in(&[
            (
                "Makefile",
                "include some_dir\n\nhelp: ## root help\n\t@echo root\nbuild:\n\t@echo build\n",
            ),
            ("some_dir/.keep", ""),
        ]);

        let targets: Vec<&str> = results.iter().map(|(target, _)| target.as_str()).collect();
        assert!(
            targets.contains(&"help") && targets.contains(&"build"),
            "expected top-level targets to survive a directory include, got {targets:?}"
        );
    }

    /// Security: completion may run inside an untrusted repository. A hostile Makefile must not be
    /// able to make tab-completion read and parse files outside the working tree via an absolute,
    /// home-relative, or `..`-escaping `include`. This builds a project whose Makefile tries the
    /// absolute and parent-traversal escape vectors against real sentinel files placed *outside*
    /// the project dir, and asserts none of their targets surface while the in-tree target does.
    #[test]
    fn test_list_targets_command_ignores_includes_outside_project() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        // Sentinels outside the project dir: one reached by absolute path, one via `..`. They are
        // real, readable files so the test proves the generator *chose* not to follow them rather
        // than them merely being absent. Unique names keep parallel test runs from colliding.
        let abs_sentinel = std::env::temp_dir().join(format!("cs_make_secret_abs_{nanos}.mk"));
        let parent_name = format!("cs_make_secret_parent_{nanos}.mk");
        let parent_sentinel = std::env::temp_dir().join(&parent_name);
        std::fs::write(
            &abs_sentinel,
            "leaked-abs: ## must not surface\n\t@echo x\n",
        )
        .unwrap();
        std::fs::write(
            &parent_sentinel,
            "leaked-parent: ## must not surface\n\t@echo y\n",
        )
        .unwrap();

        // cwd at runtime is the project dir under temp_dir, so `../<name>` resolves to the parent
        // sentinel sitting beside it in temp_dir.
        let makefile = format!(
            "include {abs}\ninclude ../{parent}\n\nhelp: ## root help\n\t@echo root\n",
            abs = abs_sentinel.display(),
            parent = parent_name,
        );
        let results = run_list_targets_in(&[("Makefile", makefile.as_str())]);

        std::fs::remove_file(&abs_sentinel).ok();
        std::fs::remove_file(&parent_sentinel).ok();

        let targets: Vec<&str> = results.iter().map(|(target, _)| target.as_str()).collect();
        assert!(
            targets.contains(&"help"),
            "in-tree target must still surface, got {targets:?}"
        );
        assert!(
            !targets.contains(&"leaked-abs"),
            "absolute include outside the tree must not be followed, got {targets:?}"
        );
        assert!(
            !targets.contains(&"leaked-parent"),
            "`..`-escaping include must not be followed, got {targets:?}"
        );
    }

    /// A line beginning with a tab is a recipe line (a shell command), not an `include` directive:
    /// GNU make only treats `include` as a directive when it is not recipe-indented. A hostile
    /// Makefile could otherwise hide an `include`-looking recipe line inside a target body to make
    /// completion follow a path make itself would never include. Assert tab-indented
    /// `include`-looking lines are not followed, while a genuine space-indented directive still is.
    #[test]
    fn test_list_targets_command_ignores_recipe_indented_include() {
        let results = run_list_targets_in(&[
            (
                "Makefile",
                // `\tinclude sneaky.mk` is a recipe line of `help`, not a directive.
                "help: ## ok\n\tinclude sneaky.mk\nbuild:\n\t@echo build\n  include real.mk\n",
            ),
            (
                "sneaky.mk",
                "recipe-leaked: ## must not surface\n\t@echo bad\n",
            ),
            ("real.mk", "real-target: ## from a space-indented include\n"),
        ]);

        let targets: Vec<&str> = results.iter().map(|(target, _)| target.as_str()).collect();
        assert!(
            targets.contains(&"help") && targets.contains(&"build"),
            "top-level targets must surface, got {targets:?}"
        );
        assert!(
            targets.contains(&"real-target"),
            "a genuine space-indented `include` must still be followed, got {targets:?}"
        );
        assert!(
            !targets.contains(&"recipe-leaked"),
            "a tab-indented (recipe) `include` line must not be followed, got {targets:?}"
        );
    }

    /// The lexical `safe()` guard cannot catch an include whose path is in-tree *textually* but
    /// resolves out of tree through a symlink (e.g. a repo shipping `escape -> /some/outside/dir`
    /// and `include escape/secret.mk`). This is what the `realpath` containment layer is for. Build
    /// a project with exactly that shape — a real out-of-tree sentinel reached only via an in-tree
    /// symlink — and assert its target never surfaces while an ordinary in-tree include still does.
    #[cfg(unix)]
    #[test]
    fn test_list_targets_command_blocks_symlink_escape() {
        use super::LIST_TARGETS_COMMAND;
        use std::process::Command;
        use std::sync::atomic::{AtomicU64, Ordering};
        use std::time::{SystemTime, UNIX_EPOCH};

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);

        let base = std::env::temp_dir();
        let project = base.join(format!("cs_make_symlink_proj_{nanos}_{seq}"));
        // The sentinel lives OUTSIDE the project, reachable only by resolving the symlink.
        let outside = base.join(format!("cs_make_symlink_outside_{nanos}_{seq}"));
        std::fs::create_dir_all(&project).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(
            outside.join("secret.mk"),
            "leaked-symlink: ## must not surface\n\t@echo bad\n",
        )
        .unwrap();
        std::fs::write(project.join("ok.mk"), "in-tree-target: ## ok\n").unwrap();
        std::fs::write(
            project.join("Makefile"),
            "include escape/secret.mk\ninclude ok.mk\n\nhelp: ## root\n\t@echo root\n",
        )
        .unwrap();
        // `escape` is an in-tree name pointing at the out-of-tree dir.
        std::os::unix::fs::symlink(&outside, project.join("escape")).unwrap();

        let output = Command::new("sh")
            .arg("-c")
            .arg(LIST_TARGETS_COMMAND)
            .current_dir(&project)
            .output()
            .expect("failed to run list_targets command");

        std::fs::remove_dir_all(&project).ok();
        std::fs::remove_dir_all(&outside).ok();

        assert!(
            output.status.success(),
            "command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let targets: Vec<(String, Option<String>)> =
            list_targets_post_process(&String::from_utf8_lossy(&output.stdout))
                .suggestions
                .into_iter()
                .map(|s| (s.exact_string, s.description))
                .collect();
        let names: Vec<&str> = targets.iter().map(|(t, _)| t.as_str()).collect();

        assert!(
            names.contains(&"help") && names.contains(&"in-tree-target"),
            "in-tree targets must surface, got {names:?}"
        );
        assert!(
            !names.contains(&"leaked-symlink"),
            "an include resolving out of tree through a symlink must not be followed, got {names:?}"
        );
    }
}
