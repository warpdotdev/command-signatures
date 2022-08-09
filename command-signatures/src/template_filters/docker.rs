use warp_completion_metadata::{TemplateFilters, TemplateFilter};

pub fn filter() -> TemplateFilters {
    TemplateFilters::new("docker-compose").add_filter(
        "filter-docker-files",
        TemplateFilter(|suggestion| {
            if suggestion.exact_string.ends_with(".yaml") || suggestion.exact_string.ends_with(".yml") {
                Some(suggestion)
            } else {
                None
            }
        })
    )
}