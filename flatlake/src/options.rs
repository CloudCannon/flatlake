use anyhow::{bail, Result};
use clap::Parser;
use schematic::{derive_enum, Config, ConfigEnum};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf};

use crate::logging::{LogLevel, Logger};

derive_enum!(
    #[derive(ConfigEnum, Default, Copy)]
    pub enum SortDirection {
        #[default]
        Asc,
        Desc,
    }
);

#[derive(ConfigEnum, Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputElement {
    Data,
    Content,
    ContentAst,
}

#[derive(Config, Debug, Clone)]
#[config(rename_all = "snake_case")]
pub struct LakeParameters {
    /// The location of your input files
    #[setting(env = "FLATLAKE_SOURCE")]
    pub source: PathBuf,

    /// Where to output the built API
    #[setting(default = "api", env = "FLATLAKE_DEST")]
    pub dest: PathBuf,

    #[setting(nested)]
    pub collections: Vec<LakeCollection>,

    #[setting(nested)]
    pub global: GlobalLakeSettings,

    /// Print verbose logging while building. Does not impact the output files.
    #[setting(env = "FLATLAKE_VERBOSE")]
    pub verbose: bool,

    ///Path to a logfile to write to. Will replace the file on each run
    #[setting(env = "FLATLAKE_LOGFILE")]
    pub logfile: Option<PathBuf>,
}

#[derive(Config, Debug, Clone)]
#[config(rename_all = "snake_case")]
pub struct LakeCollection {
    pub output_key: String,
    pub path: String,
    pub glob: String,
    #[setting(default = "date")]
    pub sort_key: String,
    #[setting(default = "asc")]
    pub sort_direction: SortDirection,
    #[setting(default = vec![OutputElement::Data, OutputElement::Content])]
    pub single_elements: Vec<OutputElement>,
    #[setting(default = vec![OutputElement::Data])]
    pub list_elements: Vec<OutputElement>,
    pub page_size: Option<usize>,
}

#[derive(Config, Debug, Clone)]
#[config(rename_all = "snake_case")]
pub struct GlobalLakeSettings {
    #[setting(default = "date")]
    pub sort_key: String,
    #[setting(default = "asc")]
    pub sort_direction: SortDirection,
    #[setting(default = 100)]
    pub page_size: usize,
}

// The configuration object used internally
#[derive(Debug, Clone)]
pub struct LakeContext {
    pub version: &'static str,
    pub logger: Logger,
    pub working_directory: PathBuf,
    pub params: LakeParameters,
}

impl LakeContext {
    pub fn load(mut config: LakeParameters) -> Result<Self> {
        let log_level = if config.verbose {
            LogLevel::Verbose
        } else {
            LogLevel::Standard
        };

        let working_directory = env::current_dir().unwrap();

        config.source = working_directory.join(PathBuf::from(config.source.clone()));
        config.dest = working_directory.join(PathBuf::from(config.dest.clone()));

        Ok(Self {
            working_directory,
            version: env!("CARGO_PKG_VERSION"),
            logger: Logger::new(log_level, true, config.logfile.clone().map(PathBuf::from)),
            params: config,
        })
    }
}
