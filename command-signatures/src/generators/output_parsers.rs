use serde_json::Value;
use warp_completion_metadata::{GeneratorResults, GeneratorResultsCollector, Suggestion};

/// One suggestion per non-empty trimmed line.
pub fn lines(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn lines_desc_extension(output: &str) -> GeneratorResults {
    split_on(output, ';', "Extension")
}

pub fn lines_desc_runtime(output: &str) -> GeneratorResults {
    split_on(output, ',', "Runtime")
}

pub fn lines_desc_instance(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Instance name")
}

pub fn lines_desc_context(output: &str) -> GeneratorResults {
    skip_header_first_token(output, 1, "Context")
}

pub fn lines_desc_warp_point(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter_map(|line| line.split_whitespace().next())
        .map(|name| Suggestion::with_description(name, "Warp point"))
        .collect_unordered_results()
}

pub fn json_envs(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("envs").cloned()) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                value
                    .as_str()
                    .map(|name| Suggestion::with_description(name, "Environment"))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_name_summary(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let name = value.get("name")?.as_str()?;
                let description = value.get("summary").and_then(Value::as_str).unwrap_or("");
                Some(Suggestion::with_description(name, description))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_string_array(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| value.as_str().map(Suggestion::new))
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_object_key_descriptions(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(name, value)| {
                let description = match value {
                    Value::String(s) => s,
                    other => other.as_str().map(str::to_owned).unwrap_or_default(),
                };
                Suggestion::with_description(name, description)
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn tailscale_peers(output: &str) -> GeneratorResults {
    tailscale_peers_with_suffix(output, "")
}

pub fn tailscale_peers_colon(output: &str) -> GeneratorResults {
    tailscale_peers_with_suffix(output, ":")
}

pub fn op_accounts(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let email = value.get("email")?.as_str()?;
                let url = value.get("url").and_then(Value::as_str).unwrap_or("");
                let uuid = value.get("account_uuid")?.as_str()?;
                Some(
                    Suggestion::with_description(uuid, url)
                        .with_display_name(Some(email.to_string())),
                )
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn projj_cache_repos(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(key, value)| {
                let name = key.rsplit('/').next().unwrap_or(&key);
                let description = value.get("repo").and_then(Value::as_str).unwrap_or("");
                Suggestion::with_description(name, description)
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn projj_hooks(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("hooks").cloned()) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(name, value)| {
                let description = match value {
                    Value::String(s) => s,
                    other => other.to_string(),
                };
                Suggestion::with_description(name, description)
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn trex_imports(output: &str) -> GeneratorResults {
    if output.trim().is_empty() {
        return empty();
    }
    match json(output).and_then(|v| v.get("imports").cloned()) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(name, value)| {
                let description = value.as_str().unwrap_or("");
                Suggestion::with_description(name, description)
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn trex_scripts(output: &str) -> GeneratorResults {
    if output.trim().is_empty() {
        return empty();
    }
    match json(output).and_then(|v| v.get("scripts").cloned()) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(name, _)| Suggestion::with_description(name, "trex script"))
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn turbo_pipeline(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("pipeline").cloned()) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(name, value)| {
                let mut parts = Vec::new();
                if let Some(Value::Array(depends)) = value.get("dependsOn") {
                    let items: Vec<_> = depends
                        .iter()
                        .filter_map(Value::as_str)
                        .map(|item| format!("'{item}'"))
                        .collect();
                    if !items.is_empty() {
                        parts.push(format!("depends on {}", items.join(", ")));
                    }
                }
                if let Some(Value::Array(outputs)) = value.get("outputs") {
                    let items: Vec<_> = outputs
                        .iter()
                        .filter_map(Value::as_str)
                        .map(|item| format!("'{item}'"))
                        .collect();
                    if !items.is_empty() {
                        parts.push(format!("outputs {}", items.join(", ")));
                    }
                }
                let description = if parts.is_empty() {
                    "Task".to_string()
                } else {
                    format!("Task: {}", parts.join(", "))
                };
                Suggestion::with_description(name, description)
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn cargo_read_manifest_bins(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("targets").cloned()) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|target| {
                let is_bin = target
                    .get("kind")
                    .and_then(Value::as_array)
                    .is_some_and(|kind| kind.iter().any(|item| item.as_str() == Some("bin")));
                is_bin
                    .then(|| {
                        target
                            .get("name")
                            .and_then(Value::as_str)
                            .map(Suggestion::new)
                    })
                    .flatten()
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_cordova_platforms(output: &str) -> GeneratorResults {
    match json(output)
        .and_then(|v| v.get("cordova").cloned())
        .and_then(|cordova| cordova.get("platforms").cloned())
    {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                value
                    .as_str()
                    .map(|name| Suggestion::with_description(name, "Platform"))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_rush_projects(output: &str) -> GeneratorResults {
    if output.trim().is_empty() {
        return empty();
    }
    match json(output).and_then(|v| v.get("projects").cloned()) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                value
                    .get("packageName")
                    .and_then(Value::as_str)
                    .map(|name| Suggestion::with_description(name, "Projects"))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_deno_codes(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let name = value.get("code")?.as_str()?;
                let docs = value.get("docs").and_then(Value::as_str).unwrap_or("");
                let description = docs.split("\n\n").next().unwrap_or(docs);
                Some(Suggestion::with_description(name, description))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_nativescript_templates(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let name = value.get("name")?.as_str()?;
                Some(Suggestion::with_description(
                    format!("@nativescript/{name}"),
                    format!("Template {name}"),
                ))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn pipe_table_name_col1_desc(output: &str) -> GeneratorResults {
    pipe_table(output, 2, 0, Some(1))
}

pub fn git_oneline(output: &str) -> GeneratorResults {
    let output = output.trim_start();
    if output.starts_with("fatal:") {
        return empty();
    }
    nonempty_lines(output)
        .map(|line| {
            let (hash, rest) = line.split_at(line.len().min(7));
            Suggestion::with_description(hash, rest)
        })
        .collect_unordered_results()
}

pub fn apt_package_before_slash(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .map(|line| {
            let name = line.split_once('/').map(|(name, _)| name).unwrap_or(line);
            Suggestion::with_description(name, "Package")
        })
        .collect_unordered_results()
}

pub fn strip_star_prefix(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .map(|line| {
            let name = line
                .strip_prefix('*')
                .map(str::trim)
                .unwrap_or_else(|| line.trim());
            Suggestion::new(name)
        })
        .collect_unordered_results()
}

pub fn second_whitespace_token(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter_map(|line| {
            line.split_whitespace()
                .nth(1)
                .filter(|token| *token != "=")
                .map(Suggestion::new)
        })
        .collect_unordered_results()
}

pub fn slice2_reversed(output: &str) -> GeneratorResults {
    let names: Vec<_> = nonempty_lines(output)
        .filter_map(|line| line.get(2..))
        .map(|name| Suggestion::with_description(name, format!("Node.js {name}")))
        .collect();
    GeneratorResults {
        suggestions: names.into_iter().rev().collect(),
        is_ordered: true,
    }
}

pub fn yaml_application(output: &str) -> GeneratorResults {
    if output.trim().is_empty() {
        return empty();
    }
    for line in output.lines() {
        let line = line.trim();
        if let Some(value) = line.strip_prefix("application:") {
            let name = value.trim().trim_matches('"').trim_matches('\'');
            if !name.is_empty() {
                return std::iter::once(Suggestion::new(name)).collect_unordered_results();
            }
        }
    }
    empty()
}

pub fn descending_count(output: &str) -> GeneratorResults {
    let Ok(count) = output.trim().parse::<usize>() else {
        return empty();
    };
    (1..=count)
        .rev()
        .map(|n| Suggestion::new(n.to_string()))
        .collect_ordered_results()
}

pub fn json_script_keys(output: &str) -> GeneratorResults {
    if output.trim().is_empty() {
        return empty();
    }
    match json(output).and_then(|v| v.get("scripts").cloned()) {
        Some(Value::Object(map)) => map
            .into_iter()
            .map(|(name, value)| {
                let description = value.as_str().unwrap_or("Script");
                Suggestion::with_description(name, description)
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn npms_search_results(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("results").cloned()) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let package = value.get("package")?;
                let name = package.get("name")?.as_str()?;
                let description = package
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                Some(Suggestion::with_description(name, description))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn json_crates(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("crates").cloned()) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let name = value.get("name")?.as_str()?;
                let description = value
                    .get("description")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .or_else(|| {
                        value
                            .get("newest_version")
                            .and_then(Value::as_str)
                            .map(|version| format!("v{version}"))
                    })
                    .unwrap_or_default();
                Some(Suggestion::with_description(name, description))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn gh_repo_list_json(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let name = value.get("nameWithOwner")?.as_str()?;
                let description = value
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                Some(Suggestion::with_description(name, description))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn github_repos_json(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let name = value.get("full_name")?.as_str()?;
                let description = value
                    .get("description")
                    .and_then(Value::as_str)
                    .unwrap_or("Repository");
                Some(Suggestion::with_description(name, description))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn robot_variables(output: &str) -> GeneratorResults {
    let mut seen = std::collections::HashSet::new();
    nonempty_lines(output)
        .filter_map(|line| {
            let start = line.find("${")?;
            let rest = &line[start + 2..];
            let end = rest.find('}')?;
            let name = rest[..end].trim();
            (!name.is_empty() && seen.insert(name.to_string()))
                .then(|| Suggestion::with_description(name, "Variable"))
        })
        .collect_unordered_results()
}

pub fn robot_test_cases(output: &str) -> GeneratorResults {
    let mut seen = std::collections::HashSet::new();
    let mut in_tests = false;
    output
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.eq_ignore_ascii_case("*** Test Cases ***")
                || trimmed.eq_ignore_ascii_case("***Test Cases***")
            {
                in_tests = true;
                return None;
            }
            if in_tests && trimmed.starts_with("***") {
                in_tests = false;
                return None;
            }
            if !in_tests || line.starts_with(' ') || line.starts_with('\t') || trimmed.is_empty() {
                return None;
            }
            let name = trimmed.split('#').next().unwrap_or(trimmed).trim();
            if name.is_empty() || name.contains("  ") {
                return None;
            }
            seen.insert(name.to_string())
                .then(|| Suggestion::with_description(name, "Test case"))
        })
        .collect_unordered_results()
}

pub fn robot_tags(output: &str) -> GeneratorResults {
    let mut seen = std::collections::HashSet::new();
    nonempty_lines(output)
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed.strip_prefix("[Tags]")
        })
        .flat_map(|rest| {
            rest.split("  ")
                .map(str::trim)
                .filter(|tag| !tag.is_empty() && !tag.starts_with('#'))
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .filter_map(|tag| {
            seen.insert(tag.clone())
                .then(|| Suggestion::with_description(tag, "Tag"))
        })
        .collect_unordered_results()
}

pub fn scc_languages(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter_map(|line| {
            let (name, _) = line.rsplit_once(" (")?;
            Some(Suggestion::new(name.trim()))
        })
        .collect_unordered_results()
}

pub fn docker_from_as_names(output: &str) -> GeneratorResults {
    let re = regex::Regex::new(r"(?i)(?:as)\s+([\w:.-]+)").ok();
    nonempty_lines(output)
        .filter_map(|line| {
            re.as_ref()
                .and_then(|re| re.captures(line))
                .and_then(|c| c.get(1))
                .map(|m| Suggestion::new(m.as_str()))
        })
        .collect_unordered_results()
}

pub fn ssh_hosts(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter(|line| line.trim().starts_with("Host ") && !line.contains('*'))
        .filter_map(|line| line.split_whitespace().nth(1))
        .map(|name| Suggestion::with_description(name, "SSH host"))
        .collect_unordered_results()
}

fn nonempty_lines(output: &str) -> impl Iterator<Item = &str> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn lines_with_desc(output: &str, description: &'static str) -> GeneratorResults {
    nonempty_lines(output)
        .map(|line| Suggestion::with_description(line, description))
        .collect_unordered_results()
}

fn split_on(output: &str, sep: char, description: &'static str) -> GeneratorResults {
    output
        .split(sep)
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| Suggestion::with_description(part, description))
        .collect_unordered_results()
}

fn pipe_table(
    output: &str,
    skip: usize,
    name_col: usize,
    desc_col: Option<usize>,
) -> GeneratorResults {
    nonempty_lines(output)
        .skip(skip)
        .filter_map(|line| {
            let cols: Vec<_> = line.split('|').map(str::trim).collect();
            let name = cols.get(name_col).copied().filter(|s| !s.is_empty())?;
            match desc_col.and_then(|i| cols.get(i).copied()) {
                Some(description) if !description.is_empty() => {
                    Some(Suggestion::with_description(name, description))
                }
                _ => Some(Suggestion::new(name)),
            }
        })
        .collect_unordered_results()
}

fn skip_header_first_token(
    output: &str,
    skip: usize,
    description: &'static str,
) -> GeneratorResults {
    nonempty_lines(output)
        .skip(skip)
        .filter_map(|line| {
            let token = line.split_whitespace().next()?.replace('*', "");
            let name = token.trim();
            (!name.is_empty()).then(|| Suggestion::with_description(name, description))
        })
        .collect_unordered_results()
}

fn tailscale_peers_with_suffix(output: &str, suffix: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("Peer").cloned()) {
        Some(Value::Object(map)) => map
            .into_values()
            .filter_map(|peer| {
                let dns = peer.get("DNSName")?.as_str()?;
                let short = dns.split('.').next().unwrap_or(dns);
                let host = peer.get("HostName").and_then(Value::as_str).unwrap_or("");
                let os = peer.get("OS").and_then(Value::as_str).unwrap_or("");
                Some(
                    Suggestion::with_description(format!("{short}{suffix}"), os)
                        .with_display_name(Some(host.to_string())),
                )
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

fn json(output: &str) -> Option<Value> {
    serde_json::from_str(output.trim()).ok()
}

fn empty() -> GeneratorResults {
    GeneratorResults {
        suggestions: vec![],
        is_ordered: false,
    }
}
