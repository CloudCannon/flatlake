use std::time::Instant;

use clap::CommandFactory;
use flatlake::{LakeContext, LakeParameters, Watershed};
use miette::{Diagnostic, Report};
use schematic::{ConfigLoader, Format};

const CONFIGS: &[&str] = &[
    "flatlake.json",
    "flatlake.yml",
    "flatlake.yaml",
    "flatlake.toml",
];

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let configs: Vec<&str> = CONFIGS
        .iter()
        .filter(|c| std::path::Path::new(c).exists())
        .cloned()
        .collect();
    if configs.len() > 1 {
        eprintln!(
            "Found multiple possible config files: [{}]",
            configs.join(", ")
        );
        eprintln!("Flatlake only supports loading one configuration file format, please ensure only one file exists.");
        std::process::exit(1);
    }

    let mut loader = ConfigLoader::<LakeParameters>::new();
    for config in configs {
        if let Err(e) = loader.file(config) {
            eprintln!("Failed to load {config}:\n{e}");
            std::process::exit(1);
        }
    }

    match loader.load() {
        Err(e) => {
            eprintln!("Failed to initialize configuration:");
            eprintln!("{:?}", Report::new(e));
            std::process::exit(1);
        }
        Ok(result) => {
            if let Ok(ctx) = LakeContext::load(result.config) {
                let mut watershed = Watershed::new(ctx);
                let logger = watershed.options.logger.clone();

                watershed.run().await;

                let duration = start.elapsed();

                logger.status(&format!(
                    "Finished in {}.{:03} seconds",
                    duration.as_secs(),
                    duration.subsec_millis()
                ));
            }
        }
    }
}
