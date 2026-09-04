use serde_json::Value;
use warp_completion_metadata::{GeneratorResults, GeneratorResultsCollector, Suggestion};

/// One suggestion per non-empty trimmed line.
pub fn lines(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn named_lines(output: &str) -> GeneratorResults {
    lines(output)
}

pub fn unique_named_lines(output: &str) -> GeneratorResults {
    let mut seen = std::collections::HashSet::new();
    nonempty_lines(output)
        .filter(|&line| seen.insert(line.to_string()))
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn desc_plugin(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Plugin")
}

pub fn desc_plugin_name(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Plugin name")
}

pub fn desc_remote(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Remote")
}

pub fn desc_script(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Script")
}

pub fn desc_version(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Version")
}

pub fn desc_variable(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Variable name")
}

pub fn desc_workspace(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .map(|line| Suggestion::with_description(line.replace('*', "").trim(), "Workspace"))
        .collect_unordered_results()
}

pub fn desc_terraform_workspace(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .map(|line| {
            Suggestion::with_description(line.replace('*', "").trim(), "Terraform workspaces")
        })
        .collect_unordered_results()
}

pub fn desc_address(output: &str) -> GeneratorResults {
    if output.contains("No state file") || output.contains("Error") {
        return empty();
    }
    nonempty_lines(output)
        .map(|line| Suggestion::with_description(line.replace('*', "").trim(), "Address"))
        .collect_unordered_results()
}

pub fn git_status_short(output: &str) -> GeneratorResults {
    if output.trim_start().starts_with("fatal:") {
        return empty();
    }
    output
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let path = line.get(3..)?.trim();
            (!path.is_empty()).then(|| Suggestion::with_description(path, line))
        })
        .collect_unordered_results()
}

pub fn pre_commit_hook_ids(output: &str) -> GeneratorResults {
    yaml_pre_commit_hook_ids(output)
        .into_iter()
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn ssh_known_hosts(output: &str) -> GeneratorResults {
    let mut prefix = String::new();
    let mut seen = std::collections::HashSet::new();
    let mut hosts = Vec::new();
    for line in nonempty_lines(output) {
        if let Some(rest) = line.strip_prefix("WARP_SSH_USER_PREFIX=") {
            prefix = rest.to_string();
            continue;
        }
        for host in known_host_names(line) {
            if seen.insert(host.clone()) {
                hosts.push(host);
            }
        }
    }
    hosts
        .into_iter()
        .map(|host| Suggestion::with_description(format!("{prefix}{host}"), "SSH host"))
        .collect_unordered_results()
}

pub fn deno_binaries(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter(|line| !line.ends_with("/deno"))
        .map(|line| {
            let name = line.rsplit('/').next().unwrap_or(line);
            Suggestion::with_description(name, line)
        })
        .collect_unordered_results()
}

pub fn eslint_plugin_names(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter(|line| line.starts_with("eslint-plugin"))
        .map(|line| {
            let name = line.split_whitespace().next().unwrap_or(line);
            Suggestion::new(name.get(14..).unwrap_or(name))
        })
        .collect_unordered_results()
}

pub fn mix_help_tasks(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter_map(|line| {
            let (name, description) = match line.split_once('#') {
                Some((name, description)) => (name.trim(), description.trim()),
                None => (line.trim(), ""),
            };
            let name = name.strip_prefix("mix ").unwrap_or(name);
            (!["mix", "help", "new", "run", "iex -S mix"].contains(&name))
                .then(|| Suggestion::with_description(name, description))
        })
        .collect_unordered_results()
}

pub fn meteor_packages(output: &str) -> GeneratorResults {
    if output.contains("No such file or directory") {
        return empty();
    }
    nonempty_lines(output)
        .filter_map(|line| {
            let name = line.split('#').next().unwrap_or(line).trim();
            let name = name.split('@').next().unwrap_or(name).trim();
            (!name.is_empty()).then(|| Suggestion::new(name))
        })
        .collect_unordered_results()
}

pub fn meteor_examples(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter(|line| line.contains("github.com"))
        .map(|line| Suggestion::new(line.split(':').next().unwrap_or(line).trim()))
        .collect_unordered_results()
}

pub fn softwareupdate_labels(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter_map(|line| {
            line.strip_prefix("* Label: ").map(|name| {
                Suggestion::with_description(format!("\"{name}\""), "Available update")
                    .with_display_name(Some(name.to_string()))
            })
        })
        .collect_unordered_results()
}

pub fn networksetup_ports(output: &str) -> GeneratorResults {
    let re = regex::Regex::new(r"(?s)Hardware Port: (.*?)\n.*?Device: (.*?)(?:\n|$)").ok();
    let Some(re) = re else {
        return empty();
    };
    re.captures_iter(output)
        .filter_map(|caps| {
            Some(Suggestion::with_description(
                caps.get(2)?.as_str(),
                caps.get(1)?.as_str(),
            ))
        })
        .collect_unordered_results()
}

fn skip_star_first_token(output: &str, description: &'static str) -> GeneratorResults {
    nonempty_lines(output)
        .skip(1)
        .filter_map(|line| {
            let token = line.split_whitespace().next()?.replace('*', "");
            let name = token.trim();
            (!name.is_empty()).then(|| Suggestion::with_description(name, description))
        })
        .collect_unordered_results()
}

pub fn okteto_contexts(output: &str) -> GeneratorResults {
    skip_star_first_token(output, "Context")
}

pub fn okteto_namespaces(output: &str) -> GeneratorResults {
    skip_star_first_token(output, "Namespace")
}

pub fn redwood_scripts(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter(|line| line.ends_with(".js") || line.ends_with(".ts"))
        .map(|line| {
            let name = line.trim();
            let name = name.rsplit_once('.').map(|(stem, _)| stem).unwrap_or(name);
            Suggestion::with_description(name, "Script")
        })
        .collect_unordered_results()
}

pub fn wifi_networks(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .skip(1)
        .map(|line| Suggestion::new(line.trim()))
        .collect_unordered_results()
}

pub fn gpg_ciphers(output: &str) -> GeneratorResults {
    let Some(start) = output.find("Cypher: ").or_else(|| output.find("Cipher: ")) else {
        return empty();
    };
    let rest = &output[start + 8..];
    let end = rest.find("Hash: ").unwrap_or(rest.len());
    let list = rest[..end].split_whitespace().collect::<String>();
    list.split(',')
        .filter(|part| !part.is_empty())
        .map(Suggestion::new)
        .collect_unordered_results()
}

pub fn desc_extension(output: &str) -> GeneratorResults {
    split_on(output, ';', "Extension")
}

pub fn desc_runtime(output: &str) -> GeneratorResults {
    split_on(output, ',', "Runtime")
}

pub fn desc_instance(output: &str) -> GeneratorResults {
    lines_with_desc(output, "Instance name")
}

pub fn desc_warp_point(output: &str) -> GeneratorResults {
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

pub fn json_deno_doc_nodes(output: &str) -> GeneratorResults {
    let Some(value) = json(output) else {
        return empty();
    };
    let Some(nodes) = value.get("nodes").and_then(Value::as_array) else {
        return empty();
    };
    nodes
        .iter()
        .filter_map(|node| {
            let name = node.get("name")?.as_str()?;
            (!name.is_empty()).then(|| {
                let description = node
                    .get("jsDoc")
                    .and_then(|js_doc| js_doc.get("doc"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                Suggestion::with_description(name, description)
            })
        })
        .collect_unordered_results()
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

pub fn docker_search_names(output: &str) -> GeneratorResults {
    nonempty_lines(output)
        .filter_map(|line| {
            json(line)?
                .get("Name")
                .and_then(Value::as_str)
                .map(Suggestion::new)
        })
        .collect_unordered_results()
}

pub fn stepzen_schema_names(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                value
                    .as_str()
                    .map(|name| Suggestion::with_description(name, "StepZen endpoint"))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn stepzen_github_dirs(output: &str) -> GeneratorResults {
    match json(output) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .filter_map(|value| {
                let kind = value.get("type")?.as_str()?;
                let name = value.get("name")?.as_str()?;
                (kind == "dir" && !name.starts_with('.'))
                    .then(|| Suggestion::with_description(name, "Stepzen schema"))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn youtube_dl_flat_playlist(output: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("entries").cloned()) {
        Some(Value::Array(arr)) => arr
            .into_iter()
            .enumerate()
            .map(|(index, entry)| {
                let title = entry.get("title").and_then(Value::as_str).unwrap_or("");
                let uploader = entry.get("uploader").and_then(Value::as_str).unwrap_or("");
                let n = index + 1;
                Suggestion::with_description(n.to_string(), uploader)
                    .with_display_name(Some(format!("{n} - {title}")))
            })
            .collect_unordered_results(),
        _ => empty(),
    }
}

pub fn youtube_clipboard_url(output: &str) -> GeneratorResults {
    let value = output.trim();
    let is_youtube = value.contains("youtube.com") || value.contains("youtu.be");
    if is_youtube {
        std::iter::once(Suggestion::with_description(value, "Clipboard"))
            .collect_unordered_results()
    } else {
        empty()
    }
}

pub fn lerna_package_script_keys(output: &str) -> GeneratorResults {
    let mut names = std::collections::BTreeSet::new();
    for chunk in output.split("END") {
        let chunk = chunk.trim();
        if chunk.is_empty() {
            continue;
        }
        let Some(Value::Object(map)) = json(chunk) else {
            continue;
        };
        let Some(Value::Object(scripts)) = map.get("scripts") else {
            continue;
        };
        names.extend(scripts.keys().cloned());
    }
    names
        .into_iter()
        .map(Suggestion::new)
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

fn known_host_names(line: &str) -> Vec<String> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return Vec::new();
    }
    let mut fields = line.split_whitespace();
    let mut first = fields.next().unwrap_or("");
    if first.starts_with('@') {
        first = fields.next().unwrap_or("");
    }
    if first.is_empty() || first.starts_with("|1|") {
        return Vec::new();
    }
    first
        .split(',')
        .filter_map(|host| {
            let host = host.trim();
            let host = host
                .strip_prefix('[')
                .and_then(|h| h.split(']').next())
                .unwrap_or(host);
            if host.is_empty() || host.contains('*') {
                None
            } else {
                Some(host.to_string())
            }
        })
        .collect()
}

fn yaml_pre_commit_hook_ids(output: &str) -> Vec<String> {
    let mut ids = Vec::new();
    let mut in_repos = false;
    let mut in_hooks = false;
    let mut repos_indent = usize::MAX;
    let mut hooks_indent = usize::MAX;
    for raw in output.lines() {
        let indent = raw.chars().take_while(|c| *c == ' ' || *c == '\t').count();
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if in_repos
            && indent <= repos_indent
            && !is_sequence_item(trimmed)
            && key_name(trimmed) != "repos"
        {
            in_repos = false;
            in_hooks = false;
        }
        if in_hooks
            && indent <= hooks_indent
            && !is_sequence_item(trimmed)
            && key_name(trimmed) != "hooks"
        {
            in_hooks = false;
        }
        match key_name(trimmed) {
            "repos" => {
                in_repos = true;
                repos_indent = indent;
                in_hooks = false;
                continue;
            }
            "hooks" if in_repos => {
                in_hooks = true;
                hooks_indent = indent;
                continue;
            }
            "id" if in_hooks => {
                if let Some(id) = yaml_scalar_value(trimmed) {
                    ids.push(id);
                }
            }
            _ => {}
        }
    }
    ids
}

fn is_sequence_item(trimmed: &str) -> bool {
    trimmed == "-" || trimmed.starts_with("- ")
}

fn key_name(trimmed: &str) -> &str {
    let line = trimmed.strip_prefix("- ").unwrap_or(trimmed);
    line.split(':').next().unwrap_or("").trim()
}

fn yaml_scalar_value(trimmed: &str) -> Option<String> {
    let line = trimmed.strip_prefix("- ").unwrap_or(trimmed);
    let (_, value) = line.split_once(':')?;
    let value = value.trim();
    if value.is_empty() || value == "|" || value == ">" || value == "{" || value == "[" {
        return None;
    }
    let quote = match value.as_bytes().first() {
        Some(b) if *b == b'"' || *b == b'\'' => Some(*b as char),
        _ => None,
    };
    if let Some(quote) = quote {
        let rest = &value[1..];
        let end = rest.find(quote)?;
        let inner = &rest[..end];
        return (!inner.is_empty()).then(|| inner.to_string());
    }
    let unquoted = value
        .split_once('#')
        .map(|(code, _)| code.trim())
        .unwrap_or(value);
    (!unquoted.is_empty()).then(|| unquoted.to_string())
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

fn tailscale_peers_with_suffix(output: &str, suffix: &str) -> GeneratorResults {
    match json(output).and_then(|v| v.get("Peer").cloned()) {
        Some(Value::Object(map)) => map
            .into_values()
            .filter_map(|peer| {
                let dns = peer.get("DNSName")?.as_str()?;
                let short = dns.split('.').next().unwrap_or(dns);
                let host = peer.get("HostName").and_then(Value::as_str).unwrap_or("");
                let os = peer.get("OS").and_then(Value::as_str).unwrap_or("");
                let description = match (host.is_empty(), os.is_empty()) {
                    (true, true) => String::new(),
                    (false, true) => host.to_string(),
                    (true, false) => os.to_string(),
                    (false, false) => format!("{host} ({os})"),
                };
                Some(Suggestion::with_description(
                    format!("{short}{suffix}"),
                    description,
                ))
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
