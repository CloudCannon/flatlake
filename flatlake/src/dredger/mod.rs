use std::path::PathBuf;

use wax::{Glob, WalkEntry};

use crate::{LakeContext, Tributary, Watershed};

impl Tributary {
    pub fn new_relative_to(file_path: PathBuf, root_path: PathBuf, collection_id: usize) -> Self {
        Self {
            collection_id,
            file_path: Some(file_path),
            root_path: Some(root_path),
            output_url: None,
        }
    }
}

pub async fn walk_for_files(ctx: &LakeContext) -> Vec<Tributary> {
    let log = &ctx.logger;
    let mut tributaries = vec![];

    log.status("[Walking collections]");

    for (collection_id, collection) in ctx.params.collections.iter().enumerate() {
        let collection_source: PathBuf = ctx.params.source.join(&collection.path);
        log.v_info(format!(
            "Looking for {} in {:?}",
            collection.glob, collection_source
        ));

        if let Ok(glob) = Glob::new(&collection.glob) {
            let collection_files = glob
                .walk(&collection_source)
                .filter_map(Result::ok)
                .map(WalkEntry::into_path)
                .map(|file_path| {
                    Tributary::new_relative_to(file_path, ctx.params.source.clone(), collection_id)
                });

            tributaries.extend(collection_files);
        } else {
            log.error(format!(
                "Error: Provided glob \"{}\" did not parse as a valid glob.",
                collection.glob
            ));
            // TODO: Bubble this error back to the Node API if applicable
            std::process::exit(1);
        }
    }

    tributaries
}
