use warp_completion_metadata::{CommandGenerators, Importance, Order, Priority, TemplateFilter};

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("node").add_filter(
        "filter-node-files",
        TemplateFilter(|mut suggestion, path_type| {
            (path_type.is_folder()
                || suggestion.exact_string.ends_with(".mjs")
                || suggestion.exact_string.ends_with(".js"))
            .then(|| {
                if !path_type.is_folder() {
                    suggestion.priority = Priority::Global(Importance::More(Order(76)));
                }
                suggestion
            })
        }),
    )
}
