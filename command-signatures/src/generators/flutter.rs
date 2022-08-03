use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("flutter")
        .add_generator(
            "emulators",
            Generator::new("flutter emulators", |output| {
                BULLET_RE
                    .captures_iter(output)
                    .filter_map(|info| info.get(0).map(|capture| capture.as_str()))
                    .map(|info| info.split('•').map(str::trim))
                    .filter_map(|mut device| {
                        match (device.next(), device.next(), device.next(), device.next()) {
                            (Some(id), Some(name), Some(manufacturer), Some(platform_type)) => {
                                let description = format!(
                                    "Available emulator:\n{} • {} • {}",
                                    name, manufacturer, platform_type
                                );
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
            Generator::new("flutter channel", |output| {
                output
                    .lines()
                    .filter(|line| ENDS_WITH_WORD.is_match(line))
                    .filter_map(|line| {
                        WORD_RE.find_iter(line.trim()).next().map(|word| {
                            let description = if line.starts_with('*') {
                                "Active Channel"
                            } else {
                                "Available Channels"
                            };

                            Suggestion::with_description(word.as_str(), description)
                        })
                    })
                    .collect_unordered_results()
            }),
        )
}

lazy_static! {
    static ref BULLET_RE: Regex = Regex::new(r"(?i).*•.*").unwrap();
    static ref ENDS_WITH_WORD: Regex = Regex::new(r"\w+$").unwrap();
    static ref WORD_RE: Regex = Regex::new(r"\w+").unwrap();
}
