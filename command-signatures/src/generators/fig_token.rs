use warp_completion_metadata::CommandBuilder;

pub fn npms_search(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    curl_npms(last_token(tokens), "")
}

pub fn npms_search_create_prefix(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let q = last_token(tokens);
    if q.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        curl_npms(&format!("create-{q}"), "")
    }
}

pub fn crates_io_search(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let q = last_token(tokens);
    if q.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!(
            "curl -sfL 'https://crates.io/api/v1/crates?q={}&per_page=60'",
            urlencode(q)
        ))
    }
}

pub fn trivy_severity_remaining(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_csv(tokens, &["UNKNOWN", "LOW", "MEDIUM", "HIGH", "CRITICAL"])
}

pub fn trivy_scanners_remaining(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_csv(tokens, &["vuln", "config"])
}

pub fn trivy_pkg_types_remaining(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_csv(tokens, &["os", "library"])
}

pub fn docker_from_as(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let file = dockerfile_from_tokens(tokens);
    CommandBuilder::single_command(format!(
        r#"grep -iE 'FROM.*AS' {} 2>/dev/null"#,
        shell_single_quote(&file)
    ))
}

pub fn docker_search(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let q = last_token(tokens);
    if q.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!(
            "docker search {} --format '{{{{ json . }}}}'",
            shell_single_quote(q)
        ))
    }
}

pub fn gh_repo_list_for_owner(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let token = last_token(tokens);
    let owner = token.split('/').next().unwrap_or("");
    if owner.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!(
            "gh repo list {} --limit 9999 --json nameWithOwner,description",
            shell_single_quote(owner)
        ))
    }
}

pub fn apt_list_prefix(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let q = last_token(tokens);
    if q.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!(
            r#"apt list 2>/dev/null | awk -F/ -v p={} '$1 ~ "^"p {{ print $1 }}'"#,
            shell_single_quote(&regex_escape(q))
        ))
    }
}

pub fn cargo_test_list(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let token = last_token(tokens);
    let depth = token.split("::").filter(|s| !s.is_empty()).count().max(1);
    let last = token.split("::").last().unwrap_or("");
    CommandBuilder::single_command(format!(
        r#"cargo t -- --list 2>/dev/null | awk '/: test$/ {{ print substr($1, 1, length($1) - 1) }}' | awk -F '::' '{{ print ${depth} }}' | grep -F {query} | sort -u"#,
        depth = depth,
        query = shell_single_quote(last)
    ))
}

pub fn chown_dscl(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let has_colon = tokens.iter().any(|t| t.contains(':'));
    if has_colon {
        CommandBuilder::single_command(
            "dscl . -list /Groups PrimaryGroupID 2>/dev/null | tr -s ' ' | sort -r",
        )
    } else {
        CommandBuilder::single_command(
            "dscl . -list /Users UniqueID 2>/dev/null | tr -s ' ' | sort -r",
        )
    }
}

pub fn nx_list(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let last = last_token(tokens);
    if last.contains(':') {
        let project = last.split(':').next().unwrap_or("");
        CommandBuilder::single_command(format!("nx list {}", shell_single_quote(project)))
    } else {
        CommandBuilder::single_command("nx list")
    }
}

pub fn youtube_dl_flat_playlist(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let urls: Vec<_> = tokens
        .iter()
        .copied()
        .filter(|t| t.contains("youtube."))
        .map(shell_single_quote)
        .collect();
    if urls.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!("youtube-dl --flat-playlist -J {}", urls.join(" ")))
    }
}

pub fn deno_doc_json(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let keep = ["--private", "--builtin", "--unstable"];
    let mut args: Vec<String> = tokens
        .iter()
        .copied()
        .take(tokens.len().saturating_sub(1))
        .filter(|t| {
            (!t.starts_with('-') || keep.contains(t)) && !t.starts_with('$') && !t.starts_with('(')
        })
        .map(shell_single_quote)
        .collect();
    args.push("--json".to_string());
    CommandBuilder::single_command(format!("deno doc {}", args.join(" ")))
}

pub fn git_status_staged_or_unstaged(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    if tokens.contains(&"--staged") || tokens.contains(&"--cached") {
        CommandBuilder::single_command(
            "git --no-optional-locks status --short | sed -ne '/^M /p' -e '/A /p'",
        )
    } else {
        CommandBuilder::single_command(
            "git --no-optional-locks status --short | sed -ne '/M /p' -e '/A /p'",
        )
    }
}

pub fn eslint_env_remaining(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_csv(
        tokens,
        &[
            "browser",
            "node",
            "commonjs",
            "shared-node-browser",
            "es6",
            "es2017",
            "es2020",
            "es2021",
            "worker",
            "amd",
            "mocha",
            "jasmine",
            "jest",
            "phantomjs",
            "protractor",
            "qunit",
            "jquery",
            "prototypejs",
            "shelljs",
            "meteor",
            "mongo",
            "applescript",
            "nashorn",
            "serviceworker",
            "atomtest",
            "embertest",
            "webextensions",
            "greasemonkey",
        ],
    )
}

pub fn brew_gist_logs_actions(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let has_non_flag = tokens
        .iter()
        .skip(2)
        .any(|t| !t.starts_with('-') && !t.is_empty());
    if has_non_flag {
        remaining_csv(tokens, &["install", "install-on-request", "build-error"])
    } else {
        remaining_csv(tokens, &["cask-install", "os-version"])
    }
}

fn curl_npms(query: &str, _: &str) -> CommandBuilder {
    if query.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!(
            "curl -s -H 'Accept: application/json' 'https://api.npms.io/v2/search?size=20&q={}'",
            urlencode(query)
        ))
    }
}

fn remaining_csv(tokens: &[&str], options: &[&str]) -> CommandBuilder {
    let used: std::collections::HashSet<&str> = last_token(tokens)
        .split(',')
        .filter(|s| !s.is_empty())
        .collect();
    let left: Vec<&str> = options
        .iter()
        .copied()
        .filter(|o| !used.contains(o))
        .collect();
    if left.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!("printf '%s\\n' {}", left.join(" ")))
    }
}

fn dockerfile_from_tokens(tokens: &[&str]) -> String {
    if let Some(i) = tokens.iter().position(|t| *t == "-f" || *t == "--file") {
        if i + 1 < tokens.len() {
            return tokens[i + 1].to_string();
        }
    }
    "$PWD/Dockerfile".to_string()
}

fn last_token<'a>(tokens: &[&'a str]) -> &'a str {
    tokens.last().copied().unwrap_or("")
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}

fn regex_escape(value: &str) -> String {
    let mut out = String::new();
    for c in value.chars() {
        if "^$.*+?()[]{}|\\".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

fn urlencode(value: &str) -> String {
    let mut out = String::new();
    for b in value.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}
