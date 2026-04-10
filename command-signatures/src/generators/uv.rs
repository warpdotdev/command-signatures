use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("uv")
        .add_generator("installed_tools", installed_tools_generator())
        .add_generator("project_dependencies", project_dependencies_generator())
        .add_generator("pip_installed_packages", pip_installed_packages_generator())
        .add_generator("python_versions", python_versions_generator())
        .add_generator("installed_pythons", installed_pythons_generator())
}

/// Lists tools installed via `uv tool list`.
/// Output format: "toolname v1.2.3\n- executable1\n..."
/// We extract only the tool name lines (those starting with a non-whitespace character).
fn installed_tools_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("uv tool list 2>/dev/null"),
        |output| {
            output
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    // Skip empty lines and indented lines (executables are prefixed with "- ")
                    if trimmed.is_empty() || line.starts_with(' ') || line.starts_with('-') {
                        return None;
                    }
                    // Format: "toolname v1.2.3" — extract the tool name and version
                    let mut parts = trimmed.splitn(2, ' ');
                    let name = parts.next()?;
                    let version = parts.next().unwrap_or("");
                    Some(Suggestion::with_description(name, version))
                })
                .collect_unordered_results()
        },
    )
}

/// Lists project dependencies from pyproject.toml using Python to parse the TOML.
/// Falls back gracefully if no pyproject.toml exists.
fn project_dependencies_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command(
            "python3 -c \"import tomllib,sys,pathlib; \
             p=pathlib.Path('pyproject.toml'); \
             d=tomllib.loads(p.read_text()) if p.exists() else {}; \
             deps=d.get('project',{}).get('dependencies',[]); \
             [print(dep.split('>')[0].split('<')[0].split('=')[0].split('!')[0].split('~')[0].split('[')[0].split(';')[0].strip()) for dep in deps]\" 2>/dev/null",
        ),
        |output| {
            output
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| Suggestion::new(line.trim()))
                .collect_unordered_results()
        },
    )
}

/// Lists pip-installed packages using `uv pip list --format freeze`.
fn pip_installed_packages_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("uv pip list --format freeze 2>/dev/null"),
        |output| {
            output
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    // Format: "package==version"
                    let mut parts = trimmed.splitn(2, "==");
                    let name = parts.next()?.trim();
                    let version = parts.next().map(|v| v.trim()).unwrap_or("");
                    if name.is_empty() {
                        return None;
                    }
                    Some(Suggestion::with_description(name, version))
                })
                .collect_unordered_results()
        },
    )
}

/// Lists available Python versions from `uv python list --only-installed`.
/// Output format: "cpython-3.12.3-linux-x86_64-gnu    /path/to/python"
fn python_versions_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("uv python list --only-installed 2>/dev/null"),
        |output| {
            output
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    // Format: "cpython-3.12.3-linux-x86_64-gnu    /path/to/python"
                    let mut parts = trimmed.split_whitespace();
                    let version_str = parts.next()?;
                    let path = parts.next().unwrap_or("");
                    Some(Suggestion::with_description(version_str, path))
                })
                .collect_unordered_results()
        },
    )
}

/// Lists installed (managed) Python versions for uninstallation.
/// Same source as python_versions but specifically for `uv python uninstall`.
fn installed_pythons_generator() -> Generator {
    Generator::script(
        CommandBuilder::single_command("uv python list --only-installed 2>/dev/null"),
        |output| {
            output
                .lines()
                .filter_map(|line| {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    let mut parts = trimmed.split_whitespace();
                    let version_str = parts.next()?;
                    let path = parts.next().unwrap_or("");
                    Some(Suggestion::with_description(version_str, path))
                })
                .collect_unordered_results()
        },
    )
}
