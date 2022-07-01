use regex::Regex;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResultsCollector, Suggestion,
};

use lazy_static::lazy_static;

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("flutter").add_generator(
        "emulators",
        Generator::new("flutter emulators", |output| {
            RE.captures_iter(output)
                .filter_map(|info| info.get(0).map(|capture| capture.as_str()))
                .map(|info| info.split('•'))
                .map(|device_info| device_info.map(|info| info.trim()))
                .filter_map(|device| {
                    let device = device.take(4).collect::<Vec<&str>>();
                    if device.len() == 4 {
                        let name = format!(
                            "${} • {} • {}",
                            device.get(1).unwrap(),
                            device.get(2).unwrap(),
                            device.get(3).unwrap()
                        );
                        Some(Suggestion::with_description(name, "Available emulators"))
                    } else {
                        None
                    }
                })
                .collect_from_unordered_suggestions()
        }),
    )
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i).*•.*").unwrap();
}
