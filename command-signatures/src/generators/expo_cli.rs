use super::fig_parse;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("expo-cli")
        .add_generator(
            "sysctl_hw_ncpu",
            Generator::script(
                CommandBuilder::single_command("sysctl -n hw.ncpu"),
                fig_parse::descending_count,
            ),
        )
        .add_generator(
            "xcodebuild_ios_xcodeproj",
            Generator::script(
                CommandBuilder::single_command("xcodebuild -project ios/*.xcodeproj -list -json"),
                fig_parse::json_string_array,
            ),
        )
        .add_generator(
            "xcrun_xctrace_list_devices",
            Generator::script(
                CommandBuilder::single_command("xcrun xctrace list devices"),
                fig_parse::second_whitespace_token,
            ),
        )
}
