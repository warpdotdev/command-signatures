use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorName, GeneratorResults, GeneratorResultsCollector,
    Suggestion,
};

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref POST_PROCESS_BRANCHES_RE: Regex = Regex::new(r"\S+").unwrap();
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

fn post_process_git_for_each_ref(output: &str) -> GeneratorResults {
    output
        .split('\n')
        .filter_map(|line| {
            (!line.is_empty()).then(|| Suggestion::with_description(line.trim(), "Branch"))
        })
        .collect_from_unordered_suggestions()
}

fn post_process_branches(out: &str) -> GeneratorResults {
    let output = filter_messages(out);

    if output.starts_with("fatal:") {
        GeneratorResults::empty()
    } else {
        output
            .split('\n')
            .filter_map(|elm| {
                let mut name = elm.trim().to_owned();
                if name.is_empty() {
                    return None;
                }

                let parts = POST_PROCESS_BRANCHES_RE.find_iter(elm).collect::<Vec<_>>();
                if parts.len() > 1 {
                    if parts[0].as_str() == "*" {
                        if elm.contains("HEAD detached") {
                            // We are in a detached HEAD state.
                            return None;
                        } else {
                            return Some(Suggestion::with_description(
                                elm.replace('*', "").trim(),
                                "Current branch",
                            ));
                        }
                    } else if parts[0].as_str() == "+" {
                        let elm = elm.replace('+', "");
                        name = elm.trim().to_owned();
                    }
                }

                Some(Suggestion::with_description(name, "Branch"))
            })
            .collect_from_unordered_suggestions()
    }
}

/// Returns the commit SHA and the commit description
fn commit_line_to_suggestion(line: &str) -> Option<Suggestion> {
    line.split_once(' ')
        .map(|(name, description)| Suggestion::with_description(name, description))
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("git")
        .add_generator(
            "commits",
            Generator::new("git --no-optional-locks log --oneline", |output| {
                let output = filter_messages(output);
                if output.starts_with("fatal:") {
                    GeneratorResults::empty()
                } else {
                    output
                        .split('\n')
                        .filter_map(commit_line_to_suggestion)
                        .collect_from_unordered_suggestions()
                }
            }),
        )
        .add_generator(
            "aliases",
            Generator::new(
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
                        .collect_from_unordered_suggestions()
                },
            ),
        )
        .add_generator(
            "revs",
            Generator::new("git rev-list --all --oneline", |output| {
                let output = filter_messages(output);
                if output.starts_with("fatal:") {
                    GeneratorResults::empty()
                } else {
                    output
                        .split('\n')
                        .filter_map(commit_line_to_suggestion)
                        .collect_from_unordered_suggestions()
                }
            }),
        )
        .add_generator(
            "stashes",
            Generator::new("git --no-optional-locks stash list", |output| {
                let output = filter_messages(output);
                if output.starts_with("fatal:") {
                    GeneratorResults::empty()
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
                        .collect_from_unordered_suggestions()
                }
            }),
        )
        .add_generator(
            GeneratorName::new("treeish"),
            Generator::new(
                "git --no-optional-locks diff --cached --name-only",
                |output| {
                    let output = filter_messages(output);
                    if output.starts_with("fatal:") {
                        return GeneratorResults::empty();
                    }

                    output
                        .split('\n')
                        .map(|file| Suggestion::with_description(file, "staged file"))
                        .collect_from_unordered_suggestions()
                },
            ),
        )
        // Get all the branches from "refs/remotes". Heavily inspired by git's native completion
        // functions. See https://github.com/git/git/blob/69a9c10c95e28df457e33b3c7400b16caf2e2962/contrib/completion/git-completion.bash#L670-L676.
        .add_generator(
            "refs_remote_branches",
            Generator::new(
                r#"git for-each-ref --format="%(refname:strip=3)" --sort="refname:strip=3"
                  "refs/remotes/**" | uniq -u"#,
                post_process_git_for_each_ref,
            ),
        )
        // Get all the remote branches, which will be prefixed with their origin.Heavily inspired by
        // git native completion functions. See https://github.com/git/git/blob/69a9c10c95e28df457e33b3c7400b16caf2e2962/contrib/completion/git-completion.bash#L647-L658.
        .add_generator(
            "remote_branches",
            Generator::new(
                r#"git for-each-ref --format="%(refname:strip=2)" "refs/remotes/**""#,
                post_process_git_for_each_ref,
            ),
        )
        .add_generator(
            "local_branches",
            Generator::new("git --no-optional-locks branch --no-color", |output| {
                post_process_branches(output)
            }),
        )
        .add_generator(
            "remotes",
            Generator::new("git --no-optional-locks remote -v", |output| {
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
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "tags",
            Generator::new("git --no-optional-locks tag --list", |output| {
                output
                    .split('\n')
                    .filter_map(|line| (!line.is_empty()).then(|| Suggestion::new(line)))
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "files_for_staging",
            Generator::new("git --no-optional-locks status --short", |output| {
                let output = filter_messages(output);
                if output.starts_with("fatal:") {
                    return GeneratorResults::empty();
                }

                output
                    .split('\n')
                    .filter_map(|file| {
                        let arr = file.trim().split(' ').collect::<Vec<_>>();
                        if arr.len() > 1 {
                            Some((
                                arr.first().unwrap().to_string(),
                                arr[1..].join(" ").trim().to_string(),
                            ))
                        } else {
                            None
                        }
                    })
                    .map(|(_working, file)| Suggestion::with_description(file, "Changed file"))
                    .collect_from_unordered_suggestions()
            }),
        )
}

#[cfg(test)]
mod tests {
    use crate::generators::git::post_process_branches;
    use warp_completion_metadata::{GeneratorResults, Suggestion};

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
                    Suggestion::with_description("_release/v0.2021.04.02.14.18._00", "Branch"),
                    Suggestion::with_description("aloke/add_new_generators", "Current branch"),
                    Suggestion::with_description("aloke/add_options", "Branch"),
                    Suggestion::with_description("aloke/add_stable_release_workflow", "Branch"),
                    Suggestion::with_description("aloke/after_frame_hook", "Branch"),
                ],
                is_ordered: false,
            }
        );
    }
}
