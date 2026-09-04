use super::output_parsers;
use warp_completion_metadata::{CommandBuilder, CommandSignatureGenerators, Generator};

pub fn generator() -> CommandSignatureGenerators {
    CommandSignatureGenerators::new("wifi-password")
        .add_generator(
            "networksetup_wi_fi_getline_print_networksetup",
            Generator::script(
                CommandBuilder::single_command("networksetup -listallhardwareports | awk '/Wi-Fi/{getline; print $2}' | xargs networksetup -listpreferredwirelessnetworks"),
                output_parsers::wifi_networks,
            ),
        )
}
