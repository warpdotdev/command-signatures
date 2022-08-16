use warp_completion_metadata::{CommandGenerators, TemplateFilter};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("node").add_filter(
        "filter-node-files",
        TemplateFilter(|suggestion| {
            (suggestion.exact_string.ends_with(".mjs") || suggestion.exact_string.ends_with(".js"))
                .then(|| suggestion)
        }),
    )
}
