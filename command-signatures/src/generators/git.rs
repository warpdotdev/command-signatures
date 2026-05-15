use itertools::Itertools;
use warp_completion_metadata::{
    Alias, CommandBuilder, CommandSignatureGenerators, Generator, GeneratorName, GeneratorResults,
    GeneratorResultsCollector, IconType, Importance, Order, Priority, Suggestion,
};

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref POST_PROCESS_BRANCHES_RE: Regex = Regex::new(r"\S+").unwrap();
    static ref CONFIG_OPTIONS: HashSet<&'static str> = HashSet::from_iter([
        "advice.pushUpdateRejected",
        "advice.pushNonFFCurrent",
        "advice.pushNonFFMatching",
        "advice.pushAlreadyExists",
        "advice.pushFetchFirst",
        "advice.pushNeedsForce",
        "advice.statusHints",
        "advice.statusUoption",
        "advice.commitBeforeMerge",
        "advice.resolveConflict",
        "advice.implicitIdentity",
        "advice.detachedHead",
        "advice.amWorkDir",
        "advice.rmHints",
        "core.fileMode",
        "core.ignoreCase",
        "core.precomposeUnicode",
        "core.protectHFS",
        "core.protectNTFS",
        "core.trustctime",
        "core.checkStat",
        "core.quotePath",
        "core.eol",
        "core.safecrlf",
        "core.autocrlf",
        "core.symlinks",
        "core.gitProxy",
        "core.ignoreStat",
        "core.preferSymlinkRefs",
        "core.bare",
        "core.worktree",
        "core.logAllRefUpdates",
        "core.repositoryFormatVersion",
        "core.sharedRepository",
        "core.warnAmbiguousRefs",
        "core.compression",
        "core.looseCompression",
        "core.packedGitWindowSize",
        "core.packedGitLimit",
        "core.deltaBaseCacheLimit",
        "core.bigFileThreshold",
        "core.excludesFile",
        "core.askPass",
        "core.attributesFile",
        "core.editor",
        "core.commentChar",
        "core.packedRefsTimeout",
        "sequence.editor",
        "core.pager",
        "core.whitespace",
        "core.fsyncObjectFiles",
        "core.preloadIndex",
        "core.createObject",
        "core.notesRef",
        "core.sparseCheckout",
        "core.abbrev",
        "add.ignoreErrors",
        "alias.*",
        "am.keepcr",
        "am.threeWay",
        "apply.ignoreWhitespace",
        "apply.whitespace",
        "branch.autoSetupMerge",
        "branch.autoSetupRebase",
        "branch.<name>.remote",
        "branch.<name>.pushRemote",
        "branch.<name>.merge",
        "branch.<name>.mergeOptions",
        "branch.<name>.rebase",
        "branch.<name>.description",
        "browser.<tool>.cmd",
        "browser.<tool>.path",
        "clean.requireForce",
        "color.branch",
        "color.branch.<slot>",
        "color.diff",
        "color.diff.<slot>",
        "color.decorate.<slot>",
        "color.grep",
        "color.grep.<slot>",
        "color.interactive",
        "color.interactive.<slot>",
        "color.pager",
        "color.showBranch",
        "color.status",
        "color.status.<slot>",
        "color.ui",
        "column.ui",
        "column.branch",
        "column.clean",
        "column.status",
        "column.tag",
        "commit.cleanup",
        "commit.gpgSign",
        "commit.status",
        "commit.template",
        "credential.helper",
        "credential.useHttpPath",
        "credential.username",
        "credential.<url>.*",
        "credentialCache.ignoreSIGHUP",
        "diff.autoRefreshIndex",
        "diff.dirstat",
        "diff.statGraphWidth",
        "diff.context",
        "diff.external",
        "diff.ignoreSubmodules",
        "diff.mnemonicPrefix",
        "diff.noprefix",
        "diff.orderFile",
        "diff.renameLimit",
        "diff.renames",
        "diff.suppressBlankEmpty",
        "diff.submodule",
        "diff.wordRegex",
        "diff.<driver>.command",
        "diff.<driver>.xfuncname",
        "diff.<driver>.binary",
        "diff.<driver>.textconv",
        "diff.<driver>.wordRegex",
        "diff.<driver>.cachetextconv",
        "diff.tool",
        "diff.algorithm",
        "difftool.<tool>.path",
        "difftool.<tool>.cmd",
        "difftool.prompt",
        "fetch.recurseSubmodules",
        "fetch.fsckObjects",
        "fetch.unpackLimit",
        "fetch.prune",
        "format.attach",
        "format.numbered",
        "format.headers",
        "format.to",
        "format.cc",
        "format.subjectPrefix",
        "format.signature",
        "format.signatureFile",
        "format.suffix",
        "format.pretty",
        "format.thread",
        "format.signOff",
        "format.coverLetter",
        "filter.<driver>.clean",
        "filter.<driver>.smudge",
        "fsck.<msg-id>",
        "fsck.skipList",
        "gc.aggressiveDepth",
        "gc.aggressiveWindow",
        "gc.auto",
        "gc.autoPackLimit",
        "gc.autoDetach",
        "gc.packRefs",
        "gc.pruneExpire",
        "gc.worktreePruneExpire",
        "gc.reflogExpire",
        "gc.<pattern>.reflogExpire",
        "gc.reflogExpireUnreachable",
        "gc.<pattern>.reflogExpireUnreachable",
        "gc.rerereResolved",
        "gc.rerereUnresolved",
        "gitcvs.commitMsgAnnotation",
        "gitcvs.enabled",
        "gitcvs.logFile",
        "gitcvs.usecrlfattr",
        "gitcvs.allBinary",
        "gitcvs.dbName",
        "gitcvs.dbDriver",
        "gitcvs.dbUser",
        "gitcvs.dbPass",
        "gitcvs.dbTableNamePrefix",
        "gitweb.category",
        "gitweb.description",
        "gitweb.owner",
        "gitweb.url",
        "gitweb.avatar",
        "gitweb.blame",
        "gitweb.grep",
        "gitweb.highlight",
        "gitweb.patches",
        "gitweb.pickaxe",
        "gitweb.remote_heads",
        "gitweb.showSizes",
        "gitweb.snapshot",
        "grep.lineNumber",
        "grep.patternType",
        "grep.extendedRegexp",
        "gpg.program",
        "gui.commitMsgWidth",
        "gui.diffContext",
        "gui.displayUntracked",
        "gui.encoding",
        "gui.matchTrackingBranch",
        "gui.newBranchTemplate",
        "gui.pruneDuringFetch",
        "gui.trustmtime",
        "gui.spellingDictionary",
        "gui.fastCopyBlame",
        "gui.copyBlameThreshold",
        "gui.blamehistoryctx",
        "guitool.<name>.cmd",
        "guitool.<name>.needsFile",
        "guitool.<name>.noConsole",
        "guitool.<name>.noRescan",
        "guitool.<name>.confirm",
        "guitool.<name>.argPrompt",
        "guitool.<name>.revPrompt",
        "guitool.<name>.revUnmerged",
        "guitool.<name>.title",
        "guitool.<name>.prompt",
        "help.browser",
        "help.format",
        "help.autoCorrect",
        "help.htmlPath",
        "http.proxy",
        "http.cookieFile",
        "http.saveCookies",
        "http.sslVersion",
        "http.sslCipherList",
        "http.sslVerify",
        "http.sslCert",
        "http.sslKey",
        "http.sslCertPasswordProtected",
        "http.sslCAInfo",
        "http.sslCAPath",
        "http.sslTry",
        "http.maxRequests",
        "http.minSessions",
        "http.postBuffer",
        "http.lowSpeedLimit",
        "http.lowSpeedTime",
        "http.noEPSV",
        "http.userAgent",
        "i18n.commitEncoding",
        "i18n.logOutputEncoding",
        "imap",
        "index.version",
        "init.templateDir",
        "instaweb.browser",
        "instaweb.httpd",
        "instaweb.local",
        "instaweb.modulePath",
        "instaweb.port",
        "interactive.singleKey",
        "log.abbrevCommit",
        "log.date",
        "log.decorate",
        "log.follow",
        "log.showRoot",
        "log.mailmap",
        "mailinfo.scissors",
        "mailmap.file",
        "mailmap.blob",
        "man.viewer",
        "man.<tool>.cmd",
        "man.<tool>.path",
        "merge.conflictStyle",
        "merge.defaultToUpstream",
        "merge.ff",
        "merge.branchdesc",
        "merge.log",
        "merge.renameLimit",
        "merge.renormalize",
        "merge.stat",
        "merge.tool",
        "merge.verbosity",
        "merge.<driver>.name",
        "merge.<driver>.driver",
        "merge.<driver>.recursive",
        "mergetool.<tool>.path",
        "mergetool.<tool>.cmd",
        "mergetool.<tool>.trustExitCode",
        "mergetool.meld.hasOutput",
        "mergetool.keepBackup",
        "mergetool.keepTemporaries",
        "mergetool.writeToTemp",
        "mergetool.prompt",
        "notes.mergeStrategy",
        "notes.<name>.mergeStrategy",
        "notes.displayRef",
        "notes.rewrite.<command>",
        "notes.rewriteMode",
        "notes.rewriteRef",
        "pack.window",
        "pack.depth",
        "pack.windowMemory",
        "pack.compression",
        "pack.deltaCacheSize",
        "pack.deltaCacheLimit",
        "pack.threads",
        "pack.indexVersion",
        "pack.packSizeLimit",
        "pack.useBitmaps",
        "pack.writeBitmaps (deprecated)",
        "pack.writeBitmapHashCache",
        "pager.<cmd>",
        "pretty.<name>",
        "pull.ff",
        "pull.rebase",
        "pull.octopus",
        "pull.twohead",
        "push.default",
        "push.followTags",
        "push.gpgSign",
        "push.recurseSubmodules",
        "rebase.stat",
        "rebase.autoSquash",
        "rebase.autoStash",
        "rebase.missingCommitsCheck",
        "receive.advertiseAtomic",
        "receive.autogc",
        "receive.certNonceSeed",
        "receive.certNonceSlop",
        "receive.fsckObjects",
        "receive.fsck.<msg-id>",
        "receive.fsck.skipList",
        "receive.unpackLimit",
        "receive.denyDeletes",
        "receive.denyDeleteCurrent",
        "receive.denyCurrentBranch",
        "receive.denyNonFastForwards",
        "receive.hideRefs",
        "receive.updateServerInfo",
        "receive.shallowUpdate",
        "remote.pushDefault",
        "remote.<name>.url",
        "remote.<name>.pushurl",
        "remote.<name>.proxy",
        "remote.<name>.fetch",
        "remote.<name>.push",
        "remote.<name>.mirror",
        "remote.<name>.skipDefaultUpdate",
        "remote.<name>.skipFetchAll",
        "remote.<name>.receivepack",
        "remote.<name>.uploadpack",
        "remote.<name>.tagOpt",
        "remote.<name>.vcs",
        "remote.<name>.prune",
        "remotes.<group>",
        "repack.useDeltaBaseOffset",
        "repack.packKeptObjects",
        "repack.writeBitmaps",
        "rerere.autoUpdate",
        "rerere.enabled",
        "sendemail.identity",
        "sendemail.smtpEncryption",
        "sendemail.smtpssl (deprecated)",
        "sendemail.smtpsslcertpath",
        "sendemail.<identity>.*",
        "sendemail.aliasesFile",
        "sendemail.aliasFileType",
        "sendemail.annotate",
        "sendemail.bcc",
        "sendemail.cc",
        "sendemail.ccCmd",
        "sendemail.chainReplyTo",
        "sendemail.confirm",
        "sendemail.envelopeSender",
        "sendemail.from",
        "sendemail.multiEdit",
        "sendemail.signedoffbycc",
        "sendemail.smtpPass",
        "sendemail.suppresscc",
        "sendemail.suppressFrom",
        "sendemail.to",
        "sendemail.smtpDomain",
        "sendemail.smtpServer",
        "sendemail.smtpServerPort",
        "sendemail.smtpServerOption",
        "sendemail.smtpUser",
        "sendemail.thread",
        "sendemail.transferEncoding",
        "sendemail.validate",
        "sendemail.xmailer",
        "sendemail.signedoffcc (deprecated)",
        "showbranch.default",
        "status.relativePaths",
        "status.short",
        "status.branch",
        "status.displayCommentPrefix",
        "status.showUntrackedFiles",
        "status.submoduleSummary",
        "stash.showPatch",
        "stash.showStat",
        "submodule.<name>.path",
        "submodule.<name>.url",
        "submodule.<name>.update",
        "submodule.<name>.branch",
        "submodule.<name>.fetchRecurseSubmodules",
        "submodule.<name>.ignore",
        "tag.sort",
        "tar.umask",
        "transfer.fsckObjects",
        "transfer.hideRefs",
        "transfer.unpackLimit",
        "uploadarchive.allowUnreachable",
        "uploadpack.hideRefs",
        "uploadpack.allowTipSHA1InWant",
        "uploadpack.allowReachableSHA1InWant",
        "uploadpack.keepAlive",
        "url.<base>.insteadOf",
        "url.<base>.pushInsteadOf",
        "user.email",
        "user.name",
        "user.signingKey",
        "versionsort.prereleaseSuffix",
        "web.browser",
    ]);
}

pub fn filter_messages(out: &str) -> &str {
    if out.starts_with("warning:") || out.starts_with("error:") {
        let split: Vec<&str> = out.splitn(2, '\n').collect();
        if split.len() > 1 {
            split[1]
        } else {
            ""
        }
    } else {
        out
    }
}

fn post_process_tracked_files(output: &str) -> GeneratorResults {
    let output = filter_messages(output);
    if output.starts_with("fatal:") {
        return GeneratorResults::default();
    }

    // `git status --short -z` emits NUL-separated records of the form `XY <path>`
    // (two-byte status code + space + raw pathname, no C-style quoting). Renames
    // and copies are emitted as two records: `R  <to>\0<from>\0` — the source
    // path follows the destination and we skip it because that file no longer
    // exists on disk.
    let mut suggestions: Vec<Suggestion> = Vec::new();
    let mut records = output.split('\0').filter(|r| !r.is_empty());
    while let Some(record) = records.next() {
        let Some(path) = record.get(3..) else { continue };
        if matches!(record.as_bytes().first(), Some(b'R') | Some(b'C')) {
            records.next();
        }
        suggestions.push(
            Suggestion::with_description(path, "Changed file")
                .with_priority(Priority::Global(Importance::More(Order(100))))
                .with_icon(IconType::File),
        );
    }

    GeneratorResults {
        suggestions,
        is_ordered: false,
    }
}

fn post_process_git_for_each_ref(output: &str) -> GeneratorResults {
    output
        .split('\n')
        .unique()
        .filter(|&line| !line.is_empty())
        .map(|line| {
            Suggestion::with_description(line.trim(), "Branch").with_icon(IconType::GitBranch)
        })
        .collect_ordered_results()
}

pub fn post_process_branches(out: &str) -> GeneratorResults {
    let output = filter_messages(out);

    if output.starts_with("fatal:") {
        GeneratorResults::default()
    } else {
        output
            .lines()
            .filter_map(|elm| {
                let mut name = elm.trim().to_owned();
                if name.is_empty() || name.starts_with("HEAD") {
                    return None;
                }

                let post_process_branch = POST_PROCESS_BRANCHES_RE.find(elm);

                if let Some(post_process_branch) = post_process_branch {
                    if post_process_branch.as_str() == "*" {
                        if elm.contains("HEAD detached") {
                            // We are in a detached HEAD state.
                            return None;
                        } else {
                            return Some(Suggestion {
                                exact_string: elm.replace('*', "").trim().to_owned(),
                                display_name: None,
                                description: Some("Current branch".to_owned()),
                                priority: Priority::most_important(),
                                icon: Some(IconType::GitBranch),
                                is_hidden: false,
                            });
                        }
                    } else if post_process_branch.as_str() == "+" {
                        let elm = elm.replace('+', "");
                        name = elm.trim().to_owned();
                    }
                }

                Some(Suggestion::with_description(name, "Branch").with_icon(IconType::GitBranch))
            })
            .collect_ordered_results()
    }
}

/// Returns the commit SHA and the commit description
fn commit_line_to_suggestion(line: &str) -> Option<Suggestion> {
    line.split_once(' ')
        .map(|(name, description)| Suggestion::with_description(name, description))
}

pub fn commits_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("git --no-optional-locks log --oneline"),
        |output| {
            let output = filter_messages(output);
            if output.starts_with("fatal:") {
                GeneratorResults::default()
            } else {
                output
                    .split('\n')
                    .filter_map(commit_line_to_suggestion)
                    .collect_ordered_results()
            }
        },
    )
}

fn local_branches_command() -> CommandBuilder {
    CommandBuilder::single_command(
        "git --no-optional-locks branch --no-color --sort=-committerdate",
    )
}

fn tags_command() -> CommandBuilder {
    CommandBuilder::single_command("git --no-optional-locks tag --list --sort=-creatordate")
}

pub fn local_branches_generator() -> Generator {
    Generator::script(local_branches_command(), post_process_branches)
}

fn post_process_tags(output: &str) -> GeneratorResults {
    output
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Suggestion::with_description(line, "tag"))
        .collect_ordered_results()
}

const REFSPEC_PREFIX_MARKER: &str = "__REFSPEC_PREFIX__";

/// Detects a refspec prefix (`+`, `:`, or `+:`) on the last token.
/// Returns the prefix string, or empty if none is detected.
fn detect_refspec_prefix(tokens: &[&str], has_trailing_whitespace: bool) -> &'static str {
    if has_trailing_whitespace {
        return "";
    }
    match tokens.last() {
        Some(t) if t.starts_with('+') && t[1..].starts_with(':') => "+:",
        Some(t) if t.starts_with('+') => "+",
        Some(t) if t.starts_with(':') => ":",
        _ => "",
    }
}

/// Wraps a command to prepend a refspec-prefix marker when the last token starts
/// with `+`, `:`, or `+:`. This handles git refspec syntax where a prefix would
/// otherwise prevent branch/tag name matching.
///
/// When the prefix contains `:` (delete refspec), `colon_cmd` is used instead of
/// `default_cmd` so the generator can surface remote refs rather than local ones.
fn with_refspec_prefix_detection(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    default_cmd: CommandBuilder,
    colon_cmd: CommandBuilder,
) -> CommandBuilder {
    let prefix = detect_refspec_prefix(tokens, has_trailing_whitespace);
    if prefix.is_empty() {
        return default_cmd;
    }
    let cmd = if prefix.contains(':') {
        colon_cmd
    } else {
        default_cmd
    };
    CommandBuilder::and(
        CommandBuilder::single_command(format!("printf '{REFSPEC_PREFIX_MARKER}{prefix}\\n'")),
        cmd,
    )
}

/// Strips the refspec-prefix marker from generator output, returning the prefix
/// string to prepend to suggestions and the remaining output.
fn strip_refspec_prefix(out: &str) -> (&str, &str) {
    match out.strip_prefix(REFSPEC_PREFIX_MARKER) {
        Some(rest) => match rest.split_once('\n') {
            Some((prefix, remaining)) => (prefix, remaining),
            None => ("", out),
        },
        None => ("", out),
    }
}

/// Prepends a string to the `exact_string` of every suggestion in the results.
fn prepend_to_suggestions(prefix: &str, results: &mut GeneratorResults) {
    if !prefix.is_empty() {
        for suggestion in &mut results.suggestions {
            suggestion.exact_string = format!("{prefix}{}", suggestion.exact_string);
        }
    }
}

fn remote_branch_names_command() -> CommandBuilder {
    CommandBuilder::single_command(
        r#"git for-each-ref --format="%(refname:strip=3)" --sort="refname:strip=3" "refs/remotes/**""#,
    )
}

fn post_process_push_refspec_branches(out: &str) -> GeneratorResults {
    let (prefix, branch_output) = strip_refspec_prefix(out);
    let mut results = if prefix.contains(':') {
        // `:` prefix means delete — surface remote branch names.
        post_process_git_for_each_ref(branch_output)
    } else {
        post_process_branches(branch_output)
    };
    prepend_to_suggestions(prefix, &mut results);
    results
}

fn post_process_push_refspec_tags(out: &str) -> GeneratorResults {
    let (prefix, tag_output) = strip_refspec_prefix(out);
    let mut results = post_process_tags(tag_output);
    prepend_to_suggestions(prefix, &mut results);
    results
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("git")
        .add_generator("commits", commits_generator())
        .add_generator(
            "aliases",
            Generator::script(
                CommandBuilder::single_command("git --no-optional-locks config --get-regexp '^alias.'"),
                |output| {
                    output
                        .split('\n')
                        .filter_map(|alias_line| {
                            alias_line.strip_prefix("alias.").and_then(|rest| {
                                let mut parts = rest.splitn(2, ' ');

                                parts.next().map(|name| {
                                    Suggestion::with_description(
                                        name,
                                        format!("Alias for {}", parts.next().unwrap_or_default()),
                                    )
                                })
                            })
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "revs",
            Generator::script(CommandBuilder::single_command("git rev-list --all --oneline"), |output| {
                let output = filter_messages(output);
                if output.starts_with("fatal:") {
                    GeneratorResults::default()
                } else {
                    output
                        .split('\n')
                        .filter_map(commit_line_to_suggestion)
                        .collect_unordered_results()
                }
            }),
        )
        .add_generator(
            "stashes",
            Generator::script(CommandBuilder::single_command("git --no-optional-locks stash list"), |output| {
                let output = filter_messages(output);
                if output.starts_with("fatal:") {
                    GeneratorResults::default()
                } else {
                    output
                        .split('\n')
                        .filter_map(|file| {
                            let mut file_split = file.split(':');
                            let name = file_split.next()?;
                            let description = file_split.nth(1)?;
                            Some(Suggestion::with_description(
                                name.trim(),
                                description.trim(),
                            ))
                        })
                        .collect_unordered_results()
                }
            }),
        )
        .add_generator(
            GeneratorName::new("treeish"),
            Generator::script(
                CommandBuilder::single_command("git --no-optional-locks diff --cached --name-only"),
                |output| {
                    let output = filter_messages(output);
                    if output.starts_with("fatal:") {
                        return GeneratorResults::default();
                    }

                    output
                        .split('\n')
                        .map(|file| Suggestion {
                            exact_string: file.to_owned(),
                            display_name: None,
                            description: Some("staged file".to_owned()),
                            priority: Priority::Default,
                            icon: Some(IconType::File),
                            is_hidden: false,
                        })
                        .collect_unordered_results()
                },
            ),
        )
        // Get all the branches from "refs/remotes". Heavily inspired by git's native completion
        // functions. See https://github.com/git/git/blob/69a9c10c95e28df457e33b3c7400b16caf2e2962/contrib/completion/git-completion.bash#L670-L676.
        .add_generator(
            "refs_remote_branches",
            Generator::script(
                CommandBuilder::single_command(r#"git for-each-ref --format="%(refname:strip=3)" --sort="refname:strip=3" "refs/remotes/**""#),
                post_process_git_for_each_ref,
            ),
        )
        // Get all the remote branches, which will be prefixed with their origin.Heavily inspired by
        // git native completion functions. See https://github.com/git/git/blob/69a9c10c95e28df457e33b3c7400b16caf2e2962/contrib/completion/git-completion.bash#L647-L658.
        .add_generator(
            "remote_branches",
            Generator::script(
                CommandBuilder::single_command(r#"git for-each-ref --format="%(refname:strip=2)" "refs/remotes/**""#),
                post_process_git_for_each_ref,
            ),
        )
        .add_generator("local_branches", local_branches_generator())
        .add_generator(
            "remotes",
            Generator::script(CommandBuilder::single_command("git --no-optional-locks remote -v"), |output| {
                let mut remote_urls = output.split('\n').fold(HashMap::new(), |mut dict, line| {
                    let mut pair = line.split('\t');

                    let remote = pair.next();
                    let url = pair.next();

                    if let Some(url) = url {
                        dict.insert(remote.unwrap(), url.split(' ').next());
                    }
                    dict
                });

                remote_urls
                    .drain()
                    .map(|(remote, _url)| Suggestion::with_description(remote, "remote"))
                    .collect_unordered_results()
            }),
        )
        .add_generator("tags", Generator::script(tags_command(), post_process_tags))
        .add_generator(
            "files_for_staging",
            Generator::script(
                CommandBuilder::single_command("git --no-optional-locks status --short -z"),
                post_process_tracked_files,
            ),
        )
        .add_generator(
            "tracked_files",
            Generator::script(
                CommandBuilder::single_command("git --no-optional-locks ls-files"),
                |output| {
                    let output = filter_messages(output);
                    if output.starts_with("fatal:") {
                        return GeneratorResults::default();
                    }

                    output
                        .lines()
                        .filter(|line| !line.is_empty())
                        .map(|file| {
                            Suggestion::with_description(file, "Tracked file")
                                .with_icon(IconType::File)
                        })
                        .collect_unordered_results()
                },
            ),
        )
        .add_generator(
            "settings_generator",
            Generator::script(CommandBuilder::single_command("git config --get-regexp '.*'"), |output| {
                output
                    .trim()
                    .lines()
                    .filter_map(|line| line.split_once(' ').map(|(prefix, _)| prefix))
                    .filter(|line| !CONFIG_OPTIONS.contains(line))
                    .map(Suggestion::new)
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "get_changed_or_tracked_files",
            Generator::command_from_tokens(
                |tokens, _, _| {
                    if tokens.contains(&"--staged") || tokens.contains(&"--cached") {
                        CommandBuilder::pipe( CommandBuilder::single_command(r#"git --no-optional-locks status --short"#), CommandBuilder::single_command(r#"sed -ne '/^M /p' -e '/A /p'"#))
                    } else {
                        CommandBuilder::pipe(CommandBuilder::single_command(r#"git --no-optional-locks status --short"#), CommandBuilder::single_command(r#"sed -ne '/M /p' -e '/A /p'"#))
                    }
                },
                post_process_tracked_files,
            ),
        )
        .add_generator(
            "local_or_remote_branch",
            Generator::command_from_tokens(
                |tokens, _, _| {
                    // If the `-r` flag is specified, only surface remote branches, otherwise only
                    // surface local branches.
                    if tokens.contains(&"-r") || tokens.contains(&"--remotes") {
                        CommandBuilder::single_command("git --no-optional-locks branch -r --no-color --sort=-committerdate")
                    } else {
                        local_branches_command()
                    }
                },
                post_process_branches,
            ),
        )
        // Generators for `git push` refspec arguments. These handle refspec prefixes:
        // `+` for force-push, `:` for deleting a remote ref, and `+:` for force-delete.
        // The prefix is detected in the current token and prepended to branch/tag suggestions
        // so the completer's prefix matcher can match correctly. When `:` is present, remote
        // branches are suggested instead of local branches.
        .add_generator(
            "push_refspec_branches",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| {
                    with_refspec_prefix_detection(
                        tokens,
                        has_trailing_whitespace,
                        local_branches_command(),
                        remote_branch_names_command(),
                    )
                },
                post_process_push_refspec_branches,
            ),
        )
        .add_generator(
            "push_refspec_tags",
            Generator::command_from_tokens(
                |tokens, has_trailing_whitespace, _| {
                    with_refspec_prefix_detection(
                        tokens,
                        has_trailing_whitespace,
                        tags_command(),
                        tags_command(),
                    )
                },
                post_process_push_refspec_tags,
            ),
        )
        .add_alias(
            "alias",
            Alias::new(
                |tokens| {
                    tokens
                        .last()
                        .map(|token| format!("git config --get alias.{}", token))
                        .unwrap_or_default()
                },
                |output, tokens, idx| {
                    Some(match output.strip_prefix('!') {
                        Some(full_replace) => full_replace.to_string(),
                        None => tokens
                            .iter()
                            .enumerate()
                            .map(|(curr_idx, token)| {
                                if curr_idx == idx {
                                    output.trim()
                                } else {
                                    token
                                }
                            })
                            .join(" "),
                    })
                },
            ),
        )
}

#[cfg(test)]
mod tests {
    use crate::generators::git::{
        detect_refspec_prefix, post_process_branches, post_process_push_refspec_branches,
        post_process_push_refspec_tags, post_process_tags, post_process_tracked_files,
    };
    use warp_completion_metadata::{
        GeneratorResults, IconType, Importance, Order, Priority, Suggestion,
    };

    #[test]
    fn test_post_process_branches() {
        let command_output = r"_release/v0.2021.04.02.14.18._00
* aloke/add_new_generators
  aloke/add_options
  aloke/add_stable_release_workflow
  aloke/after_frame_hook";

        assert_eq!(
            post_process_branches(command_output),
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "_release/v0.2021.04.02.14.18._00".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/add_new_generators".to_owned(),
                        display_name: None,
                        description: Some("Current branch".to_owned()),
                        priority: Priority::most_important(),
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/add_options".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/add_stable_release_workflow".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/after_frame_hook".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    fn changed_file(path: &str) -> Suggestion {
        Suggestion {
            exact_string: path.to_owned(),
            display_name: None,
            description: Some("Changed file".to_owned()),
            priority: Priority::Global(Importance::More(Order(100))),
            icon: Some(IconType::File),
            is_hidden: false,
        }
    }

    #[test]
    fn test_post_process_tracked_files() {
        // `git status --short -z` output: NUL-separated records, each `XY <path>`.
        let command_output =
            " M app/src/features.rs\0M  app/src/launch_config_palette.rs\0 M app/src/workspace/mod.rs\0";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![
                    changed_file("app/src/features.rs"),
                    changed_file("app/src/launch_config_palette.rs"),
                    changed_file("app/src/workspace/mod.rs"),
                ],
                is_ordered: false,
            }
        );
    }

    /// Filenames with spaces must be preserved intact. Under `-z` git emits raw
    /// bytes with no C-style quoting, so the parser must take everything after
    /// the 3-byte `XY ` prefix rather than splitting on whitespace.
    #[test]
    fn test_post_process_tracked_files_with_spaces_in_path() {
        // Untracked file `new file test.csv` under `-z`:
        let command_output = "?? new file test.csv\0";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![changed_file("new file test.csv")],
                is_ordered: false,
            }
        );
    }

    /// Renames under `-z` are emitted as two records: `R  <to>\0<from>\0`.
    /// We surface the destination only — the source no longer exists on disk.
    #[test]
    fn test_post_process_tracked_files_rename() {
        let command_output = "R  new name.txt\0old name.txt\0 M other.rs\0";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![changed_file("new name.txt"), changed_file("other.rs")],
                is_ordered: false,
            }
        );
    }

    /// Copies (`C`) are formatted the same way as renames (`<to>\0<from>\0`)
    /// and must skip the source record just like renames.
    #[test]
    fn test_post_process_tracked_files_copy() {
        let command_output = "C  copied.txt\0source.txt\0";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![changed_file("copied.txt")],
                is_ordered: false,
            }
        );
    }

    /// Two renames in a row exercise the iterator-state interaction between
    /// successive skip-source decisions.
    #[test]
    fn test_post_process_tracked_files_back_to_back_renames() {
        let command_output = "R  a.rs\0a-old.rs\0R  b.rs\0b-old.rs\0";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![changed_file("a.rs"), changed_file("b.rs")],
                is_ordered: false,
            }
        );
    }

    /// `git status --short -z` emits untracked directories with a trailing slash.
    #[test]
    fn test_post_process_tracked_files_untracked_directory() {
        let command_output = "?? dir with space/\0";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![changed_file("dir with space/")],
                is_ordered: false,
            }
        );
    }

    /// Empty output yields no suggestions.
    #[test]
    fn test_post_process_tracked_files_empty() {
        assert_eq!(
            post_process_tracked_files(""),
            GeneratorResults {
                suggestions: vec![],
                is_ordered: false,
            }
        );
    }

    /// Fatal errors short-circuit to the default (empty, ordered) result.
    #[test]
    fn test_post_process_tracked_files_fatal_error() {
        let command_output = "fatal: not a git repository\n";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults::default()
        );
    }

    #[test]
    fn test_post_process_tags() {
        let command_output = "v1.0.0\nv2.0.0\nv0.1.0";
        assert_eq!(
            post_process_tags(command_output),
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "v1.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "v2.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "v0.1.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    #[test]
    fn test_post_process_tags_filters_empty_lines() {
        let command_output = "v1.0.0\n\nv2.0.0\n";
        assert_eq!(post_process_tags(command_output).suggestions.len(), 2);
    }

    #[test]
    fn test_detect_refspec_prefix() {
        assert_eq!(
            detect_refspec_prefix(&["git", "push", "origin", "+main"], false),
            "+"
        );
        assert_eq!(
            detect_refspec_prefix(&["git", "push", "origin", ":main"], false),
            ":"
        );
        assert_eq!(
            detect_refspec_prefix(&["git", "push", "origin", "+:main"], false),
            "+:"
        );
        assert_eq!(
            detect_refspec_prefix(&["git", "push", "origin", "main"], false),
            ""
        );
        // Trailing whitespace means no prefix (user finished the token).
        assert_eq!(
            detect_refspec_prefix(&["git", "push", "origin", "+main"], true),
            ""
        );
    }

    #[test]
    fn test_push_refspec_branches_without_prefix() {
        // Without any marker, results should match normal branch processing.
        let command_output = "* main\n  feature/foo\n  develop";
        assert_eq!(
            post_process_push_refspec_branches(command_output),
            post_process_branches(command_output),
        );
    }

    #[test]
    fn test_push_refspec_branches_with_force_prefix() {
        // With the `+` refspec prefix marker, all branch exact_strings should be prefixed with "+".
        let command_output = "__REFSPEC_PREFIX__+\n* main\n  feature/foo\n  develop";
        let results = post_process_push_refspec_branches(command_output);
        assert_eq!(
            results,
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "+main".to_owned(),
                        display_name: None,
                        description: Some("Current branch".to_owned()),
                        priority: Priority::most_important(),
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "+feature/foo".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "+develop".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    #[test]
    fn test_push_refspec_branches_with_colon_prefix() {
        // With the `:` refspec prefix marker, output is from `git for-each-ref` (remote branches)
        // and all suggestions should be prefixed with ":".
        let command_output = "__REFSPEC_PREFIX__:\nmain\nfeature/foo\ndevelop";
        let results = post_process_push_refspec_branches(command_output);
        assert_eq!(
            results,
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: ":main".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: ":feature/foo".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: ":develop".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    #[test]
    fn test_push_refspec_branches_with_force_colon_prefix() {
        // With the `+:` refspec prefix marker (force-delete), output is from `git for-each-ref`
        // and all suggestions should be prefixed with "+:".
        let command_output = "__REFSPEC_PREFIX__+:\nmain\nfeature/bar";
        let results = post_process_push_refspec_branches(command_output);
        assert_eq!(
            results,
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "+:main".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "+:feature/bar".to_owned(),
                        display_name: None,
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    #[test]
    fn test_push_refspec_tags_without_prefix() {
        let command_output = "v1.0.0\nv2.0.0";
        let results = post_process_push_refspec_tags(command_output);
        assert_eq!(
            results,
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "v1.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "v2.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    #[test]
    fn test_push_refspec_tags_with_force_prefix() {
        let command_output = "__REFSPEC_PREFIX__+\nv1.0.0\nv2.0.0";
        let results = post_process_push_refspec_tags(command_output);
        assert_eq!(
            results,
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "+v1.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "+v2.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }

    #[test]
    fn test_push_refspec_tags_with_colon_prefix() {
        let command_output = "__REFSPEC_PREFIX__:\nv1.0.0\nv2.0.0";
        let results = post_process_push_refspec_tags(command_output);
        assert_eq!(
            results,
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: ":v1.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: ":v2.0.0".to_owned(),
                        display_name: None,
                        description: Some("tag".to_owned()),
                        priority: Priority::Default,
                        icon: None,
                        is_hidden: false,
                    },
                ],
                is_ordered: true,
            }
        );
    }
}
