use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("expo-cli")
        .add_generator(
            "sysctl_hw_ncpu",
            Generator::script(
                CommandBuilder::single_command("sysctl -n hw.ncpu"),
                output_parsers::descending_count,
            ),
        )
        .add_generator(
            "xcodebuild_ios_xcodeproj",
            Generator::script(
                CommandBuilder::single_command("xcodebuild -project ios/*.xcodeproj -list -json"),
                output_parsers::json_string_array,
            ),
        )
        .add_generator(
            "xcrun_xctrace_list_devices",
            Generator::script(
                CommandBuilder::single_command("xcrun xctrace list devices"),
                output_parsers::second_whitespace_token,
            ),
        )
        .add_generator(
            "npms_search",
            Generator::command_from_tokens(
                super::fig_token::npms_search,
                output_parsers::npms_search_results,
            ),
        )
}
