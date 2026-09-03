use warp_completion_metadata::{
    Importance, Order, PathSuggestionType, Priority, Suggestion, TemplateFilter,
};

pub fn py() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["py"]))
}

pub fn yml_yaml() -> TemplateFilter {
    TemplateFilter(|suggestion, path_type| folders_or_ext(suggestion, path_type, &["yml", "yaml"]))
}

pub fn envrc() -> TemplateFilter {
    TemplateFilter(|mut suggestion, path_type| {
        let is_envrc = suggestion.exact_string.contains(".envrc");
        if path_type.is_folder() || is_envrc {
            if is_envrc {
                suggestion.priority = Priority::Global(Importance::More(Order(76)));
            }
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

#[allow(dead_code)]
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

fn folders_or_ext(
    suggestion: Suggestion,
    path_type: PathSuggestionType,
    extensions: &[&str],
) -> Option<Suggestion> {
    if path_type.is_folder() {
        return Some(suggestion);
    }
    let name = suggestion.exact_string.to_ascii_lowercase();
    extensions
        .iter()
        .any(|ext| name.ends_with(&format!(".{ext}")) || name.ends_with(&format!(".{ext}/")))
        .then_some(suggestion)
}
