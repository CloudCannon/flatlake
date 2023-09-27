use std::path::PathBuf;

use futures::future::join_all;
use logging::Logger;
use options::SortDirection;
pub use options::{LakeContext, LakeParameters};
use serde::Serialize;
use serde_json::Value;

use crate::{aggregates::generate_all_aggregate_files, dredger::walk_for_files};

mod aggregates;
mod dredger;
mod logging;
mod options;
mod outflow;
mod sampler;

pub struct Watershed {
    pub options: LakeContext,
}

#[derive(Debug)]
pub struct Tributary {
    pub collection_id: usize,
    pub file_path: Option<PathBuf>,
    /// Built URLs should be relative to this directory
    pub root_path: Option<PathBuf>,
    pub output_url: Option<String>,
}

#[derive(Debug)]
pub struct DataPoint {
    pub collection_id: usize,
    pub output_url: PathBuf,
    pub front_matter: Option<serde_json::Value>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DataPointReference<'a> {
    pub url: &'a PathBuf,
    pub data: &'a Value,
}

#[derive(Debug)]
pub struct AggregateDataPoints {
    pub sort_key: String,
    pub sort_direction: SortDirection,
    pub output_url: PathBuf,
    pub lists: Vec<usize>,
}

impl Watershed {
    pub fn new(options: LakeContext) -> Self {
        Self { options }
    }

    pub async fn run(&mut self) {
        println!("flatlake running as {}", env!("CARGO_PKG_VERSION"));

        let files = walk_for_files(&self.options).await;

        self.options
            .logger
            .v_info(format!("Found {} processable file(s)", files.len()));

        let sampling: Vec<_> = files
            .into_iter()
            .map(|f| f.read_file(&self.options))
            .collect();
        let sampled_files = join_all(sampling)
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let aggs = generate_all_aggregate_files(&self.options, &sampled_files).await;
        let writing_aggs: Vec<_> = aggs
            .into_iter()
            .map(|agg| outflow::create_list_output(agg, &sampled_files, &self.options))
            .collect();
        join_all(writing_aggs).await;

        let writing: Vec<_> = sampled_files
            .into_iter()
            .map(|dp| outflow::create_output(dp, &self.options))
            .collect();
        join_all(writing).await;

        println!("finished running flatlake");
    }
}