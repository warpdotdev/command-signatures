use warp_completion_metadata::{
    CommandGenerators, Generator, GeneratorResults, GeneratorResultsCollector, Suggestion,
};

use serde_json::Result;

#[derive(serde::Deserialize)]
struct Deployment {
    #[serde(default)]
    name: String,
}

#[derive(serde::Deserialize)]
struct Table {
    #[serde(default)]
    rows: Option<Vec<Deployment>>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
struct BoshDeployment {
    #[serde(default)]
    tables: Option<Vec<Table>>,
}

pub fn generator() -> CommandGenerators {
    CommandGenerators::new("bosh").add_generator(
        "deployments",
        Generator::new("bosh --json deployments", |output| {
            if output.starts_with("fatal:") {
                return GeneratorResults::default();
            }

            let deployment: Result<BoshDeployment> = serde_json::from_str(output);
            if let Ok(deployment) = deployment {
                if let Some(first_table) =
                    deployment.tables.and_then(|table| table.into_iter().next())
                {
                    if let Some(rows) = first_table.rows {
                        return rows
                            .into_iter()
                            .map(|row| Suggestion::with_description(row.name, "deployment"))
                            .collect_unordered_results();
                    }
                }
            } else {
                log::info!(
                    "Failed to serialize output for bosh {:?}",
                    deployment.err().unwrap()
                );
            }

            GeneratorResults::default()
        }),
    )
}
