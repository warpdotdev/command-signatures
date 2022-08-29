use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Importance, Order,
    Priority, Suggestion,
};

lazy_static! {
    static ref MAKE_TARGET_RE: Regex =
        Regex::new(r"(?m)((?:^#.*\n)*)(?:^\.[A-Z_]+:.*\n)*(^\S*?):.*?(?:\s#+[ \t]*(.+))?$")
            .unwrap();
    static ref STARTS_WITH_COMMENT: Regex = Regex::new(r"^#+\s*").unwrap();
    static ref SPECIAL_TARGETS: HashSet<&'static str> = HashSet::from_iter([
        ".PHONY",
        ".SUFFIXES",
        ".DEFAULT",
        ".PRECIOUS",
        ".INTERMEDIATE",
        ".SECONDARY",
        ".SECONDEXPANSION",
        ".DELETE_ON_ERROR",
        ".IGNORE",
        ".LOW_RESOLUTION_TIME",
        ".SILENT",
        ".EXPORT_ALL_VARIABLES",
        ".NOTPARALLEL",
        ".ONESHELL",
        ".POSIX",
    ]);
}

fn list_targets_post_process(output: &str) -> GeneratorResults {
    MAKE_TARGET_RE
        .captures_iter(output)
        .filter_map(|capture| {
            let entire_match = capture.get(0)?.as_str();
            if SPECIAL_TARGETS.contains(entire_match) {
                return None;
            }

            let leading_comment = capture.get(1)?.as_str();
            let target = capture.get(2)?.as_str();
            // The regex may not have a match for the capture group matching inline_comment. Both
            // target and leading comment should always have a match.
            let inline_comment = capture.get(3);

            // Determine what the description should be based on the present of either a leading
            // comment or inline comment on the target. If neither exist, fallback to "Make target"
            // as the description
            let description = match (inline_comment, leading_comment) {
                (Some(inline_comment), _) if !inline_comment.as_str().is_empty() => {
                    inline_comment.as_str().into()
                }
                (_, leading_comment) if !leading_comment.is_empty() => {
                    STARTS_WITH_COMMENT.replace_all(leading_comment, "")
                }
                // If there are no comments, fallback to `Make target` as the description.
                _ => "Make target".into(),
            };

            Some(
                Suggestion::with_description(target.trim(), description.trim())
                    .with_priority(Priority::Global(Importance::More(Order(80)))),
            )
        })
        .collect_unordered_results()
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("make").add_generator(
        "list_targets",
        Generator::script("cat [Mm]akefile", list_targets_post_process),
    )
}

#[cfg(test)]
mod tests {
    use crate::generators::make::list_targets_post_process;

    #[test]
    fn test_list_targets_generator() {
        let output = r#"TARGET = alacritty

ASSETS_DIR = extra
RELEASE_DIR = target/release
MANPAGE = $(ASSETS_DIR)/alacritty.man
MANPAGE-MSG = $(ASSETS_DIR)/alacritty-msg.man
TERMINFO = $(ASSETS_DIR)/alacritty.info
COMPLETIONS_DIR = $(ASSETS_DIR)/completions
COMPLETIONS = $(COMPLETIONS_DIR)/_alacritty \
	$(COMPLETIONS_DIR)/alacritty.bash \
	$(COMPLETIONS_DIR)/alacritty.fish

APP_NAME = Alacritty.app
APP_TEMPLATE = $(ASSETS_DIR)/osx/$(APP_NAME)
APP_DIR = $(RELEASE_DIR)/osx
APP_BINARY = $(RELEASE_DIR)/$(TARGET)
APP_BINARY_DIR = $(APP_DIR)/$(APP_NAME)/Contents/MacOS
APP_EXTRAS_DIR = $(APP_DIR)/$(APP_NAME)/Contents/Resources
APP_COMPLETIONS_DIR = $(APP_EXTRAS_DIR)/completions

DMG_NAME = Alacritty.dmg
DMG_DIR = $(RELEASE_DIR)/osx

vpath $(TARGET) $(RELEASE_DIR)
vpath $(APP_NAME) $(APP_DIR)
vpath $(DMG_NAME) $(APP_DIR)

all: help

help: ## Print this help message
	@grep -E '^[a-zA-Z._-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

## Build a release binary
binary: $(TARGET)-native
binary-universal: $(TARGET)-universal ## Build a universal release binary

app: $(APP_NAME)-native ## Create an Alacritty.app
app-universal: $(APP_NAME)-universal

dmg: $(DMG_NAME)-native"#;

        let generator_results = list_targets_post_process(output);
        let name_and_description: Vec<_> = generator_results
            .suggestions
            .into_iter()
            .map(|suggestion| (suggestion.exact_string, suggestion.description))
            .collect();

        assert_eq!(
            name_and_description,
            [
                ("all", Some("Make target")),
                ("help", Some("Print this help message")),
                ("binary", Some("Build a release binary")),
                ("binary-universal", Some("Build a universal release binary")),
                ("app", Some("Create an Alacritty.app")),
                ("app-universal", Some("Make target")),
                ("dmg", Some("Make target")),
            ]
            .into_iter()
            .map(|(name, description)| (name.to_owned(), description.map(ToOwned::to_owned)))
            .collect::<Vec<_>>()
        );
    }
}
