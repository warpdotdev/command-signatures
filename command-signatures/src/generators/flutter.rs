use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("flutter")
        .add_generator(
            "emulators",
            Generator::script("flutter emulators", |output| {
                output
                    .lines()
                    .filter(|line| line.contains('•'))
                    .map(|info| info.split('•').map(str::trim))
                    .filter_map(|mut device| {
                        match (device.next(), device.next(), device.next(), device.next()) {
                            (Some(id), Some(name), Some(manufacturer), Some(platform_type)) => {
                                let description =
                                    format!("{} • {} • {}", name, manufacturer, platform_type);
                                Some(Suggestion::with_description(id, description))
                            }
                            _ => None,
                        }
                    })
                    .collect_unordered_results()
            }),
        )
        .add_generator(
            "channels",
            Generator::script("flutter channel", |output| {
                output
                    .lines()
                    .filter(|line| ENDS_WITH_WORD.is_match(line))
                    .filter_map(|line| {
                        let line = line.trim();
                        line.split_whitespace().next_back().map(|word| {
                            let description = if line.starts_with('*') {
                                "Active Channel"
                            } else {
                                "Available Channels"
                            };

                            Suggestion::with_description(word, description)
                        })
                    })
                    .collect_unordered_results()
            }),
        )
}

lazy_static! {
    static ref ENDS_WITH_WORD: Regex = Regex::new(r"\w+$").unwrap();
}
