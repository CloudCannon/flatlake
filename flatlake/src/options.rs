use anyhow::{bail, Result};
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
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
    FlatData,
    Content,
    ContentAst,
}

#[derive(ConfigEnum, Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputMethod {
    Single,
    List,
    Aggregate,
}

pub fn get_cli_matches() -> ArgMatches {
    command!()
        .arg(
            arg!(
                -s --source <DIR> "The location of your source files"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -d --dest <DIR> "The location Flatlake should write your output files. Defaults to `api`"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -v --verbose ... "Print verbose logging while generating files. Does not affect the contents of the output files"
            )
            .action(clap::ArgAction::SetTrue),
        )
        .arg(
            arg!(
                --logfile <DIR> "Path to a logfile to write to. Will replace the file on each run"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
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

    /// Print verbose logging while building. Does not impact the output files.
    #[setting(env = "FLATLAKE_VERBOSE")]
    pub verbose: bool,

    ///Path to a logfile to write to. Will replace the file on each run
    #[setting(env = "FLATLAKE_LOGFILE")]
    pub logfile: Option<PathBuf>,

    #[setting(nested)]
    pub collections: Vec<LakeCollection>,

    #[setting(nested)]
    pub global: GlobalLakeSettings,
}

#[derive(Config, Debug, Clone)]
#[config(rename_all = "snake_case")]
pub struct LakeCollection {
    pub output_key: String,
    #[setting(nested)]
    pub inputs: Vec<LakeCollectionInput>,
    #[setting(default = vec![OutputElement::Data, OutputElement::Content])]
    pub single_elements: Vec<OutputElement>,
    #[setting(default = vec![OutputElement::Data])]
    pub list_elements: Vec<OutputElement>,
    pub outputs: Option<Vec<OutputMethod>>,
    pub sort_key: Option<String>,
    pub sort_direction: Option<SortDirection>,
    pub page_size: Option<usize>,
}

#[derive(Config, Debug, Clone)]
#[config(rename_all = "snake_case")]
pub struct LakeCollectionInput {
    pub path: String,
    #[setting(default = "**/*.{md}")]
    pub glob: String,
    pub sub_key: Option<String>,
    pub merge_data: Option<serde_json::Map<String, serde_json::Value>>,
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
    #[setting(default = vec![OutputMethod::Single, OutputMethod::List, OutputMethod::Aggregate])]
    pub outputs: Vec<OutputMethod>,
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

impl LakeParameters {
    pub fn override_from_cli(&mut self, cli_matches: ArgMatches) {
        if cli_matches.get_flag("verbose") {
            self.verbose = true;
        }

        if let Some(source) = cli_matches.get_one::<PathBuf>("source") {
            self.source = source.clone();
        }

        if let Some(dest) = cli_matches.get_one::<PathBuf>("dest") {
            self.dest = dest.clone();
        }

        if let Some(logfile) = cli_matches.get_one::<PathBuf>("logfile") {
            self.logfile = Some(logfile.clone());
        }
    }
}
