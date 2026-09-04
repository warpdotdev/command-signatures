use warp_completion_metadata::{
    Importance, Order, PathSuggestionType, Priority, Suggestion, TemplateFilter,
};

pub fn py() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["py"], Some(76)))
}

pub fn yml_yaml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["yml", "yaml"], None)
    })
}

pub fn envrc() -> TemplateFilter {
    TemplateFilter(|mut suggestion, path_type| {
        if path_type.is_folder() {
            return Some(suggestion);
        }
        let name = suggestion.exact_string.as_str();
        let is_envrc = name == ".envrc"
            || name == ".envrc/"
            || name.ends_with("/.envrc")
            || name.ends_with("/.envrc/");
        if is_envrc {
            suggestion.priority = Priority::Global(Importance::More(Order(76)));
            Some(suggestion)
        } else {
            None
        }
    })
}

pub fn tsconfig_json() -> TemplateFilter {
    TemplateFilter(|mut suggestion, path_type| {
        let name = suggestion.exact_string.as_str();
        let is_json = name.ends_with(".json") || name.ends_with(".json/");
        if path_type.is_folder() || is_json {
            if name.ends_with("tsconfig.json") || name.ends_with("tsconfig.json/") {
                suggestion.priority = Priority::Global(Importance::More(Order(100)));
            } else if is_json {
                suggestion.priority = Priority::Global(Importance::More(Order(76)));
            }
            Some(suggestion)
        } else {
            None
        }
    })
}

pub fn xcodeproj() -> TemplateFilter {
    TemplateFilter(|mut suggestion, _path_type| {
        if suggestion.exact_string.ends_with(".xcodeproj/")
            || suggestion.exact_string.ends_with(".xcodeproj")
        {
            suggestion.priority = Priority::Global(Importance::More(Order(76)));
        }
        Some(suggestion)
    })
}

pub fn rs() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["rs"], None))
}

pub fn cargo_toml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_equals(suggestion, path_type, &["Cargo.toml"])
    })
}

pub fn cargo_lock() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_equals(suggestion, path_type, &["Cargo.lock"])
    })
}

pub fn rustfmt_toml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_equals(suggestion, path_type, &["rustfmt.toml"])
    })
}

pub fn js_ts_family() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(
            suggestion,
            path_type,
            &["ts", "tsx", "js", "jsx", "mjs", "cjs"],
            None,
        )
    })
}

pub fn deployctl() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(
            suggestion,
            path_type,
            &["js", "mjs", "jsx", "mjsx", "ts", "mts", "tsx", "mtsx"],
            None,
        )
    })
}

pub fn exs() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["exs"], Some(76))
    })
}

pub fn java_class() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["java", "class"], None)
    })
}

pub fn jar() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["jar"], None))
}

pub fn jl() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["jl"], Some(76)))
}

pub fn dylib_so_dll() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["dylib", "so", "dll"], Some(76))
    })
}

pub fn json() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["json"], None))
}

pub fn yaml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["yaml"], None))
}

pub fn yaml_json() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["yaml", "json"], None)
    })
}

pub fn json_yaml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["json", "yaml"], None)
    })
}

pub fn scpt() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["scpt", "scptd"], Some(76))
    })
}

pub fn pdf() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["pdf"], None))
}

pub fn robot() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["robot"], Some(76))
    })
}

pub fn xml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["xml"], None))
}

pub fn py_yaml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["py", "yaml"], None)
    })
}

pub fn zip() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["zip"], None))
}

pub fn config_js() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        if path_type.is_folder() {
            return Some(suggestion);
        }
        suggestion
            .exact_string
            .ends_with("config.js")
            .then_some(suggestion)
    })
}

pub fn r_src() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(
            suggestion,
            path_type,
            &["c", "cc", "cpp", "m", "mm", "M", "f", "f90", "f95"],
            None,
        )
    })
}

pub fn r_archive() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        if path_type.is_folder() {
            return Some(suggestion);
        }
        let name = suggestion.exact_string.to_ascii_lowercase();
        ["tar", "tar.gz", "tzr.bz2", "tar.xz", "tgz"]
            .iter()
            .any(|ext| name.ends_with(&format!(".{ext}")) || name.ends_with(&format!(".{ext}/")))
            .then_some(suggestion)
    })
}

pub fn rd() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["rd"], Some(76)))
}

pub fn r() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["r"], Some(76)))
}

pub fn shortcut() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["shortcut"], None)
    })
}

pub fn sqlite() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(
            suggestion,
            path_type,
            &["sql", "sqlite", "sqlite3", "db"],
            None,
        )
    })
}

pub fn sublime() -> TemplateFilter {
    TemplateFilter(|mut suggestion, path_type| {
        if path_type.is_folder() {
            return Some(suggestion);
        }
        let name = suggestion.exact_string.as_str();
        let matched = name.ends_with(".sublime-project")
            || name.ends_with(".sublime-workspace")
            || name.ends_with(".sublime-project/")
            || name.ends_with(".sublime-workspace/");
        if matched {
            suggestion.priority = Priority::Global(Importance::More(Order(76)));
            Some(suggestion)
        } else {
            None
        }
    })
}

pub fn ts() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["ts"], Some(76)))
}

pub fn ts_tsx() -> TemplateFilter {
    TemplateFilter(|mut suggestion, path_type| {
        if path_type.is_folder() {
            return Some(suggestion);
        }
        let name = suggestion.exact_string.to_ascii_lowercase();
        let matched = name.ends_with(".ts")
            || name.ends_with(".tsx")
            || name.ends_with(".ts/")
            || name.ends_with(".tsx/");
        if matched {
            suggestion.priority = Priority::Global(Importance::More(Order(76)));
            Some(suggestion)
        } else {
            None
        }
    })
}

pub fn vsix() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["vsix"], None))
}

pub fn wasm() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["wasm"], None))
}

pub fn podspec() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| {
        folders_or_ext(suggestion, path_type, &["podspec"], None)
    })
}

fn folders_or_ext(
    mut suggestion: Suggestion,
    path_type: PathSuggestionType,
    extensions: &[&str],
    file_priority: Option<u32>,
) -> Option<Suggestion> {
    if path_type.is_folder() {
        return Some(suggestion);
    }
    let name = suggestion.exact_string.to_ascii_lowercase();
    let matched = extensions.iter().any(|ext| {
        let ext = ext.to_ascii_lowercase();
        name.ends_with(&format!(".{ext}")) || name.ends_with(&format!(".{ext}/"))
    });
    if matched {
        if let Some(priority) = file_priority {
            suggestion.priority = Priority::Global(Importance::More(Order(priority)));
        }
        Some(suggestion)
    } else {
        None
    }
}

fn folders_or_equals(
    suggestion: Suggestion,
    path_type: PathSuggestionType,
    names: &[&str],
) -> Option<Suggestion> {
    if path_type.is_folder() {
        return Some(suggestion);
    }
    let value = suggestion.exact_string.as_str();
    names
        .iter()
        .any(|name| {
            value == *name
                || value.ends_with(&format!("/{name}"))
                || value.ends_with(&format!("{name}/"))
                || value.ends_with(name)
        })
        .then_some(suggestion)
}
