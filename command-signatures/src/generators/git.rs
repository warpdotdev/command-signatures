use warp_completion_metadata::{
    CommandSignatureGenerators, Generator, GeneratorName, GeneratorResults, GeneratorResultsCollector,
    IconType, Importance, Order, Priority, Suggestion,
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

fn filter_messages(out: &str) -> &str {
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

    output
        .lines()
        // The first non-whitespace string is just a character indicating the type of indexed file.
        .filter_map(|file| file.split_whitespace().nth(1))
        .map(|file| {
            let mut suggestion = Suggestion::with_description(file, "Changed file");
            suggestion.priority = Priority::Global(Importance::More(Order(100)));
            suggestion.icon = Some(IconType::File);
            suggestion
        })
        .collect_unordered_results()
}

fn post_process_git_for_each_ref(output: &str) -> GeneratorResults {
    output
        .split('\n')
        .filter_map(|line| {
            (!line.is_empty()).then(|| Suggestion {
                exact_string: line.trim().to_owned(),
                description: Some("Branch".to_owned()),
                priority: Priority::Default,
                icon: Some(IconType::GitBranch),
                is_hidden: false,
            })
        })
        .collect_ordered_results()
}

fn post_process_branches(out: &str) -> GeneratorResults {
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

                Some(Suggestion {
                    exact_string: name,
                    description: Some("Branch".to_owned()),
                    priority: Priority::Default,
                    icon: Some(IconType::GitBranch),
                    is_hidden: false,
                })
            })
            .collect_ordered_results()
    }
}

/// Returns the commit SHA and the commit description
fn commit_line_to_suggestion(line: &str) -> Option<Suggestion> {
    line.split_once(' ')
        .map(|(name, description)| Suggestion::with_description(name, description))
}

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("git")
        .add_generator(
            "commits",
            Generator::script("git --no-optional-locks log --oneline", |output| {
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
            "aliases",
            Generator::script(
                "git --no-optional-locks config --get-regexp '^alias.'",
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
            Generator::script("git rev-list --all --oneline", |output| {
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
            Generator::script("git --no-optional-locks stash list", |output| {
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
                "git --no-optional-locks diff --cached --name-only",
                |output| {
                    let output = filter_messages(output);
                    if output.starts_with("fatal:") {
                        return GeneratorResults::default();
                    }

                    output
                        .split('\n')
                        .map(|file| Suggestion {
                            exact_string: file.to_owned(),
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
                r#"git for-each-ref --format="%(refname:strip=3)" --sort="refname:strip=3" \
                "refs/remotes/**" | uniq -u"#,
                post_process_git_for_each_ref,
            ),
        )
        // Get all the remote branches, which will be prefixed with their origin.Heavily inspired by
        // git native completion functions. See https://github.com/git/git/blob/69a9c10c95e28df457e33b3c7400b16caf2e2962/contrib/completion/git-completion.bash#L647-L658.
        .add_generator(
            "remote_branches",
            Generator::script(
                r#"git for-each-ref --format="%(refname:strip=2)" "refs/remotes/**""#,
                post_process_git_for_each_ref,
            ),
        )
        .add_generator(
            "local_branches",
            Generator::script(
                "git --no-optional-locks branch --no-color --sort=-committerdate",
                post_process_branches,
            ),
        )
        .add_generator(
            "remotes",
            Generator::script("git --no-optional-locks remote -v", |output| {
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
        .add_generator(
            "tags",
            Generator::script(
                "git --no-optional-locks tag --list --sort=-committerdate",
                |output| {
                    output
                        .lines()
                        .filter_map(|line| (!line.is_empty()).then(|| Suggestion::new(line)))
                        .collect_ordered_results()
                },
            ),
        )
        .add_generator(
            "files_for_staging",
            Generator::script(
                "git --no-optional-locks status --short",
                post_process_tracked_files,
            ),
        )
        .add_generator(
            "settings_generator",
            Generator::script("git config --get-regexp '.*'", |output| {
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
                |context| {
                    if context.contains(&"--staged") || context.contains(&"--cached") {
                        "git --no-optional-locks status --short | sed -ne '/^M /p' -e '/A /p'"
                            .to_string()
                    } else {
                        "git --no-optional-locks status --short | sed -ne '/M /p' -e '/A /p'"
                            .to_string()
                    }
                },
                post_process_tracked_files,
            ),
        )
        .add_generator(
            "local_or_remote_branch",
            Generator::command_from_tokens(
                |context| {
                    // If the `-r` flag is specified, only surface remote branches, otherwise only
                    // surface local branches.
                    let command = if context.contains(&"-r") || context.contains(&"--remotes") {
                        "git --no-optional-locks branch -r --no-color --sort=-committerdate"
                    } else {
                        "git --no-optional-locks branch --no-color --sort=-committerdate"
                    };

                    command.into()
                },
                post_process_branches,
            ),
        )
}

#[cfg(test)]
mod tests {
    use crate::generators::git::{post_process_branches, post_process_tracked_files};
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
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/add_new_generators".to_owned(),
                        description: Some("Current branch".to_owned()),
                        priority: Priority::most_important(),
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/add_options".to_owned(),
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/add_stable_release_workflow".to_owned(),
                        description: Some("Branch".to_owned()),
                        priority: Priority::Default,
                        icon: Some(IconType::GitBranch),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "aloke/after_frame_hook".to_owned(),
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
    fn test_post_process_tracked_files() {
        let command_output = r"
         M app/src/features.rs
        M  app/src/launch_config_palette.rs
         M app/src/workspace/mod.rs";

        assert_eq!(
            post_process_tracked_files(command_output),
            GeneratorResults {
                suggestions: vec![
                    Suggestion {
                        exact_string: "app/src/features.rs".to_owned(),
                        description: Some("Changed file".to_owned()),
                        priority: Priority::Global(Importance::More(Order(100))),
                        icon: Some(IconType::File),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "app/src/launch_config_palette.rs".to_owned(),
                        description: Some("Changed file".to_owned()),
                        priority: Priority::Global(Importance::More(Order(100))),
                        icon: Some(IconType::File),
                        is_hidden: false,
                    },
                    Suggestion {
                        exact_string: "app/src/workspace/mod.rs".to_owned(),
                        description: Some("Changed file".to_owned()),
                        priority: Priority::Global(Importance::More(Order(100))),
                        icon: Some(IconType::File),
                        is_hidden: false,
                    },
                ],
                is_ordered: false,
            }
        );
    }
}
