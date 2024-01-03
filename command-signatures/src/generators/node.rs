use warp_completion_metadata::{CommandSignatureGenerators, Priority, TemplateFilter};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("node").add_filter(
        "filter-node-files",
        TemplateFilter(|mut suggestion, path_type| {
            (path_type.is_folder()
                || suggestion.exact_string.ends_with(".mjs")
                || suggestion.exact_string.ends_with(".js"))
            .then(|| {
                if !path_type.is_folder() {
                    suggestion.priority = Priority::new(176);
                }
                suggestion
            })
        }),
    )
}
