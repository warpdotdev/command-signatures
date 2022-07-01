use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Result;
use std::collections::HashMap;
use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

lazy_static! {
    static ref LIST_RE: Regex = Regex::new(r"^(List)|\*").unwrap();
    static ref DEVICE_RE: Regex = Regex::new("device$").unwrap();
    static ref IOS_GET_DEVICES_RE: Regex = Regex::new(r"\([\w\d\-]+\)$").unwrap();
    static ref GRADLE_TASKS_RE: Regex = Regex::new(r"^\w+ \- |\*/").unwrap();
}

#[derive(serde::Deserialize)]
struct XcodeBuildProject {
    #[serde(default)]
    schemes: Vec<String>,

    #[serde(default)]
    configurations: Vec<String>,
}

#[derive(serde::Deserialize)]
struct XcodeBuildOutput {
    project: XcodeBuildProject,
}

#[derive(serde::Deserialize)]
struct XcRunDevice {
    name: String,
}

#[derive(serde::Deserialize)]
struct XcRunOutput {
    devices: HashMap<String, Vec<XcRunDevice>>,
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("react-native")
        .add_generator(
            "worker_generator",
            Generator::new("sysctl -n hw.ncpu", |output| {
                if let Ok(val) = output.parse::<usize>() {
                    (0..val)
                        .map(|val| Suggestion::new(val.to_string()))
                        .collect_from_unordered_suggestions()
                } else {
                    GeneratorResults::empty()
                }
            }),
        )
        .add_generator(
            "xcode_config_generator",
            Generator::new(
                "xcodebuild -project ios/*.xcodeproj  -list -json",
                |output| {
                    let json_output: Result<XcodeBuildOutput> = serde_json::from_str(output);
                    match json_output {
                        Ok(configurations) => configurations
                            .project
                            .configurations
                            .into_iter()
                            .map(Suggestion::new)
                            .collect_from_unordered_suggestions(),
                        Err(e) => {
                            log::info!("Unable to deserialize xcode build output: {:?}", e);
                            GeneratorResults::empty()
                        }
                    }
                },
            ),
        )
        .add_generator(
            "xcode_scheme_generator",
            Generator::new(
                "xcodebuild -project ios/*.xcodeproj  -list -json",
                |output| {
                    let json_output: Result<XcodeBuildOutput> = serde_json::from_str(output);
                    match json_output {
                        Ok(configurations) => configurations
                            .project
                            .schemes
                            .into_iter()
                            .map(Suggestion::new)
                            .collect_from_unordered_suggestions(),
                        Err(e) => {
                            log::info!("Unable to deserialize xcode build output: {:?}", e);
                            GeneratorResults::empty()
                        }
                    }
                },
            ),
        )
        .add_generator(
            "android_get_devices_generator",
            Generator::new("adb devices", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        if line.is_empty() || LIST_RE.is_match(line) {
                            return None;
                        }

                        if DEVICE_RE.is_match(line) {
                            if let Some(item) = line.split("device").next() {
                                return Some(Suggestion::new(item.trim()));
                            }
                        }

                        None
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "ios_get_devices_simulator_generator",
            Generator::new("xcrun simctl list --json devices available", |output| {
                let json_output: Result<XcRunOutput> = serde_json::from_str(output);
                match json_output {
                    Ok(xc_run_output) => xc_run_output
                        .devices
                        .into_iter()
                        .flat_map(|(_, devices)| devices.into_iter())
                        .map(|device| Suggestion::new(device.name))
                        .collect_from_unordered_suggestions(),
                    Err(e) => {
                        log::info!("Unable to deserialize xcrun output: {:?}", e);
                        GeneratorResults::empty()
                    }
                }
            }),
        )
        .add_generator(
            "ios_get_devices_generator",
            Generator::new("xcrun xctrace list devices", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        if !line.is_empty() && !line.starts_with('=') {
                            if let Some(name) = IOS_GET_DEVICES_RE.split(line).next() {
                                return Some(Suggestion::new(name.trim()));
                            }
                        }
                        None
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "ios_get_devices_udid_generator",
            Generator::new("xcrun xctrace list devices", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        if !line.starts_with('=') && !line.is_empty() {
                            let words = line.split("").collect::<Vec<_>>();
                            words.last().map(|word| {
                                let name = word.trim().replace('(', "").replace(')', "");
                                Suggestion::new(name)
                            })
                        } else {
                            None
                        }
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
        .add_generator(
            "gradle_tasks_generator",
            Generator::new("cd android/ && ./gradlew tasks", |output| {
                output
                    .split('\n')
                    .filter_map(|line| {
                        if GRADLE_TASKS_RE.is_match(line) {
                            let name_and_description: Vec<&str> = line.split(" - ").collect();

                            if name_and_description.len() >= 2 {
                                return Some(Suggestion::with_description(
                                    name_and_description[0],
                                    name_and_description[1],
                                ));
                            }
                        }
                        None
                    })
                    .collect_from_unordered_suggestions()
            }),
        )
}

#[cfg(test)]
mod tests {
    use crate::generators::react_native::generator;
    use warp_completion_metadata::{GeneratorName, GeneratorResults, Suggestion};

    #[test]
    fn test_ios_devices_generator() {
        let output = r"== Devices ==
Aloke’s MacBook Pro (19880D93-154D-5EC3-A788-EB281A82FD8B)

== Simulators ==
Apple TV Simulator (15.0) (3878BEF3-F561-4E3A-9597-7829CD7666C1)
Apple TV 4K (2nd generation) Simulator (15.0) (DF7E1802-6461-4A9A-BD1E-9C4AFE41FF28)";

        let output = generator()
            .generators()
            .get(&GeneratorName::from("ios_get_devices_generator"))
            .unwrap()
            .on_complete(output);
        assert_eq!(
            output,
            GeneratorResults {
                suggestions: vec![
                    Suggestion::new("Aloke’s MacBook Pro"),
                    Suggestion::new("Apple TV Simulator (15.0)"),
                    Suggestion::new("Apple TV 4K (2nd generation) Simulator (15.0)")
                ],
                is_ordered: false
            }
        )
    }

    #[test]
    fn test_ios_devices_simulator_generator() {
        let output = r#"{
  "devices" : {
    "com.apple.CoreSimulator.SimRuntime.watchOS-7-0" : [

    ],
    "com.apple.CoreSimulator.SimRuntime.tvOS-15-0" : [
      {
        "dataPath" : "\/Users\/aloke\/Library\/Developer\/CoreSimulator\/Devices\/3878BEF3-F561-4E3A-9597-7829CD7666C1\/data",
        "logPath" : "\/Users\/aloke\/Library\/Logs\/CoreSimulator\/3878BEF3-F561-4E3A-9597-7829CD7666C1",
        "udid" : "3878BEF3-F561-4E3A-9597-7829CD7666C1",
        "isAvailable" : true,
        "deviceTypeIdentifier" : "com.apple.CoreSimulator.SimDeviceType.Apple-TV-1080p",
        "state" : "Shutdown",
        "name" : "Apple TV"
      },
      {
        "dataPath" : "\/Users\/aloke\/Library\/Developer\/CoreSimulator\/Devices\/DF7E1802-6461-4A9A-BD1E-9C4AFE41FF28\/data",
        "logPath" : "\/Users\/aloke\/Library\/Logs\/CoreSimulator\/DF7E1802-6461-4A9A-BD1E-9C4AFE41FF28",
        "udid" : "DF7E1802-6461-4A9A-BD1E-9C4AFE41FF28",
        "isAvailable" : true,
        "deviceTypeIdentifier" : "com.apple.CoreSimulator.SimDeviceType.Apple-TV-4K-2nd-generation-4K",
        "state" : "Shutdown",
        "name" : "Apple TV 4K (2nd generation)"
      }
    ]
  }
}"#;
        let output = generator()
            .generators()
            .get(&GeneratorName::from("ios_get_devices_simulator_generator"))
            .unwrap()
            .on_complete(output);
        assert_eq!(
            output,
            GeneratorResults {
                suggestions: vec![
                    Suggestion::new("Apple TV"),
                    Suggestion::new("Apple TV 4K (2nd generation)"),
                ],
                is_ordered: false
            }
        )
    }
}
