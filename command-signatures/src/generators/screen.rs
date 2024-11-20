use lazy_static::lazy_static;
use warp_completion_metadata::{
    CommandBuilder, CommandSignatureGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    static ref LIST_SESSIONS_COMMAND: CommandBuilder = CommandBuilder::pipe(
        CommandBuilder::single_command("screen -ls 2"),
        CommandBuilder::pipe(
            CommandBuilder::single_command("sed '1d;$d' | sed '$d'"),
            CommandBuilder::single_command("sed '$d'")
        )
    );
}

fn list_sessions(output: &str) -> impl Iterator<Item = &str> {
    output.lines().map(str::trim)
}

pub fn generator() -> CommandSignatureGenerators {
    // For these generators, we need to skip both the last two lines and the first line of `screen -ls`
    // an example output for this command is:
    //     There are screens on:
    //         10651.ianrocks (Detached)
    //         19411.suraj_iscool  (Attached)
    //         9991.asdf  (Detached)
    //     3 Sockets in /var/folders/2j/cr14k92n1xb909k2vrq4t6sh0000gn/T/.screen.
    // Only the three middle lines in this example are relevant.
    CommandSignatureGenerators::new("screen")
        .add_generator(
            "sessions",
            Generator::script(LIST_SESSIONS_COMMAND.clone(), |output| {
                list_sessions(output)
                    .filter_map(|session_line| session_line.split('\t').next().map(Suggestion::new))
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "detached_sessions",
            Generator::script(LIST_SESSIONS_COMMAND.clone(), |output| {
                list_sessions(output)
                    .filter_map(|session_line| {
                        if !session_line.ends_with("(Detached)") {
                            return None;
                        }
                        session_line.split('\t').next().map(Suggestion::new)
                    })
                    .collect_unordered_results()
            }),
        )
}
