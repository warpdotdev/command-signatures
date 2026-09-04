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
    if let Some(i) = tokens.iter().position(|t| *t == "-f" || *t == "--file") {
        if let Some(file) = tokens.get(i + 1) {
            return CommandBuilder::single_command(format!(
                r#"grep -iE 'FROM.*AS' {} 2>/dev/null"#,
                shell_single_quote(file),
            ));
        }
    }
    CommandBuilder::single_command(r#"grep -iE 'FROM.*AS' "$PWD/Dockerfile" 2>/dev/null"#)
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

pub fn cargo_test_list(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    _: &[String],
) -> CommandBuilder {
    let token = if has_trailing_whitespace {
        ""
    } else {
        last_token(tokens)
    };
    let token = if token.starts_with('-') { "" } else { token };
    let segments = token.split("::").filter(|s| !s.is_empty()).count();
    let depth = if token.ends_with("::") {
        segments + 1
    } else {
        segments.max(1)
    };
    CommandBuilder::single_command(format!(
        r#"cargo t -- --list 2>/dev/null | awk '/: test$/ {{ print substr($1, 1, length($1) - 1) }}' | awk -F '::' -v n={depth} '{{ s=$1; for (i=2; i<=n && i<=NF; i++) s=s "::" $i; print s }}' | grep -F {query} | sort -u"#,
        depth = depth,
        query = shell_single_quote(token)
    ))
}

pub fn chown_dscl(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let last = last_token(tokens);
    if let Some((user, _)) = last.split_once(':') {
        let prefix = format!("{user}:");
        CommandBuilder::single_command(format!(
            r#"{{ getent group 2>/dev/null | cut -d: -f1; dscl . -list /Groups PrimaryGroupID 2>/dev/null | awk '{{print $1}}'; }} | awk -v p={} 'NF && !seen[$0]++ {{ print p $0 }}'"#,
            shell_single_quote(&prefix)
        ))
    } else {
        CommandBuilder::single_command(
            r#"{ getent passwd 2>/dev/null | cut -d: -f1; dscl . -list /Users UniqueID 2>/dev/null | awk '{print $1}'; } | awk 'NF && !seen[$0]++'"#,
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

pub fn deno_doc_json(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    _: &[String],
) -> CommandBuilder {
    let keep = ["--private", "--builtin", "--unstable"];
    let complete = if has_trailing_whitespace || tokens.len() <= 1 {
        tokens
    } else {
        &tokens[..tokens.len() - 1]
    };
    let mut args: Vec<String> = complete
        .iter()
        .copied()
        .filter(|t| {
            !t.is_empty()
                && (!t.starts_with('-') || keep.contains(t))
                && !t.starts_with('$')
                && !t.starts_with('(')
        })
        .map(shell_single_quote)
        .collect();
    if args.is_empty() {
        return CommandBuilder::single_command("true");
    }
    args.push("--json".to_string());
    CommandBuilder::single_command(args.join(" "))
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

pub fn known_hosts_file(
    tokens: &[&str],
    has_trailing_whitespace: bool,
    _: &[String],
) -> CommandBuilder {
    let last = if has_trailing_whitespace {
        ""
    } else {
        last_token(tokens)
    };
    let prefix = user_at_prefix(last);
    if prefix.is_empty() {
        CommandBuilder::single_command("cat ~/.ssh/known_hosts")
    } else {
        CommandBuilder::single_command(format!(
            "printf '%s\\n' {}; cat ~/.ssh/known_hosts",
            shell_single_quote(&format!("WARP_SSH_USER_PREFIX={prefix}"))
        ))
    }
}

pub fn github_user_repos(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let last = last_token(tokens);
    if last.contains(':') || !last.contains('/') {
        return CommandBuilder::single_command("true");
    }
    let user = last.split('/').next().unwrap_or("");
    if user.is_empty() {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(format!(
            "curl -sL 'https://api.github.com/users/{}/repos'",
            urlencode(user)
        ))
    }
}

pub fn git_flow_type_branches(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let kind = tokens.get(1).copied().unwrap_or("");
    if kind.is_empty() || !kind.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return CommandBuilder::single_command("true");
    }
    CommandBuilder::single_command(format!(
        r#"p=$(git config --get gitflow.prefix.{kind} 2>/dev/null); git --no-optional-locks branch -a --no-color --sort=-committerdate | sed 's/^[*+] //; s/^  //' | awk -v p="$p" 'index($0,p)==1 {{ print substr($0,length(p)+1) }}'"#
    ))
}

pub fn dd_conv_remaining(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_csv(
        tokens,
        &[
            "ascii",
            "oldascii",
            "block",
            "ebcdic",
            "ibm",
            "oldebcdic",
            "oldibm",
            "lcase",
            "noerror",
            "notrunc",
            "osync",
            "sparse",
            "swab",
            "sync",
            "ucase",
            "unblock",
        ],
    )
}

pub fn man_sections_remaining(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_delimited(tokens, &["1", "2", "3", "4", "5", "6", "7", "8"], ':')
}

pub fn file_param_keys(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let last = last_token(tokens);
    if last.contains('=') {
        CommandBuilder::single_command("true")
    } else {
        remaining_delimited(
            tokens,
            &[
                "bytes",
                "elf_notes",
                "elf_phum",
                "encoding",
                "indir",
                "name",
                "regex",
            ],
            ',',
        )
    }
}

pub fn robot_variables(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    if last_token(tokens).contains(':') {
        CommandBuilder::single_command("true")
    } else {
        CommandBuilder::single_command(
            r#"for i in $(find -E . -regex ".*.(robot|resource)" -type f); do cat -s $i ; done"#,
        )
    }
}

pub fn scc_output_paths(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    remaining_csv(
        tokens,
        &[
            "tabular",
            "wide",
            "json",
            "csv",
            "csv-stream",
            "cloc-yaml",
            "html",
            "html-table",
            "sql",
            "sql-insert",
        ],
    )
}

pub fn esbuild_loader(tokens: &[&str], _: bool, _: &[String]) -> CommandBuilder {
    let last = last_token(tokens);
    if last.contains(':') || last == "--loader" || last == "--banner" {
        remaining_delimited(
            tokens,
            &[
                "js", "jsx", "ts", "tsx", "css", "json", "text", "base64", "file", "dataurl",
                "binary", "copy",
            ],
            ':',
        )
    } else {
        CommandBuilder::single_command(
            "find . -type f -name '*.*' ! -path '*/node_modules/*' | sed 's/.*\\.//' | sort -u",
        )
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
    remaining_delimited(tokens, options, ',')
}

fn remaining_delimited(tokens: &[&str], options: &[&str], delimiter: char) -> CommandBuilder {
    let last = last_token(tokens);
    let (insert_prefix, csv) = last
        .rsplit_once('=')
        .filter(|(head, _)| {
            !head.is_empty()
                && !head.contains(delimiter)
                && head
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        })
        .map(|(head, rest)| (format!("{head}="), rest))
        .unwrap_or_else(|| (String::new(), last));
    let ends_with_delim = csv.ends_with(delimiter);
    let mut parts: Vec<&str> = csv.split(delimiter).filter(|s| !s.is_empty()).collect();
    if !csv.is_empty() && !ends_with_delim {
        parts.pop();
    }
    let used: std::collections::HashSet<&str> = parts.iter().copied().collect();
    let left: Vec<&str> = options
        .iter()
        .copied()
        .filter(|o| !used.contains(o))
        .collect();
    if left.is_empty() {
        return CommandBuilder::single_command("true");
    }
    let already = if parts.is_empty() {
        insert_prefix
    } else {
        format!(
            "{insert_prefix}{}{delimiter}",
            parts.join(&delimiter.to_string())
        )
    };
    let rendered: Vec<String> = left
        .iter()
        .map(|option| format!("{already}{option}"))
        .collect();
    CommandBuilder::single_command(format!("printf '%s\\n' {}", rendered.join(" ")))
}

fn user_at_prefix(token: &str) -> String {
    let Some((user, _)) = token.split_once('@') else {
        return String::new();
    };
    if user.is_empty()
        || !user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
    {
        return String::new();
    }
    format!("{user}@")
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
