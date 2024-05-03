use std::path::PathBuf;

use wax::{Glob, LinkBehavior, WalkBehavior, WalkEntry};

use crate::{LakeContext, Tributary, Watershed};

impl Tributary {
    pub fn new_relative_to(
        file_path: PathBuf,
        root_path: PathBuf,
        collection_id: usize,
        input_id: usize,
        collection_name: String,
    ) -> Self {
        Self {
            collection_id,
            input_id,
            collection_name,
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
        for (input_id, input) in collection.inputs.iter().enumerate() {
            let collection_source: PathBuf = ctx.params.source.join(&input.path);
            log.v_info(format!(
                "Looking for {} in {:?}",
                input.glob, collection_source
            ));

            if let Ok(glob) = Glob::new(&input.glob) {
                let collection_files = glob
                    .walk_with_behavior(
                        &collection_source,
                        WalkBehavior {
                            depth: usize::MAX,
                            link: LinkBehavior::ReadTarget,
                        },
                    )
                    .filter_map(Result::ok)
                    .map(WalkEntry::into_path)
                    .map(|file_path| {
                        Tributary::new_relative_to(
                            file_path,
                            collection_source.clone(),
                            collection_id,
                            input_id,
                            collection.output_key.clone(),
                        )
                    });

                tributaries.extend(collection_files);
            } else {
                log.error(format!(
                    "Error: Provided glob \"{}\" did not parse as a valid glob.",
                    input.glob
                ));
                // TODO: Bubble this error back to the Node API if applicable
                std::process::exit(1);
            }
        }
    }

    tributaries
}
