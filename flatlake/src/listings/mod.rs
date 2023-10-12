use el_slugify::slugify;
use std::collections::{HashMap, HashSet};

use crate::{options::SortDirection, AggregatedDataPoints, DataPoint, LakeContext};

pub async fn generate_all_listing_files(
    ctx: &LakeContext,
    data: &Vec<DataPoint>,
) -> Vec<AggregatedDataPoints> {
    let mut all_aggs = vec![];

    all_aggs.push(generate_listing_files(ctx, data, None).await);

    for i in 0..ctx.params.collections.len() {
        all_aggs.push(generate_listing_files(ctx, data, Some(i)).await);
    }

    all_aggs
}

pub async fn generate_listing_files(
    ctx: &LakeContext,
    data: &Vec<DataPoint>,
    for_collection_id: Option<usize>,
) -> AggregatedDataPoints {
    let mut page_size = ctx.params.global.page_size;
    let mut output_url = "all/".to_string();
    let mut sort_key = ctx.params.global.sort_key.clone();
    let mut sort_direction = ctx.params.global.sort_direction;

    if let Some(collection_id) = for_collection_id {
        let collection_options = ctx
            .params
            .collections
            .get(collection_id)
            .expect("Listing should match a valid collection");

        if let Some(page_size_override) = collection_options.page_size {
            page_size = page_size_override;
        }

        sort_key = collection_options.sort_key.clone();
        sort_direction = collection_options.sort_direction.clone();
        output_url = format!("{}/all/", collection_options.output_key);
    }

    let list_items = data
        .iter()
        .enumerate()
        .filter(|(_, data_point)| match for_collection_id {
            None => true,
            Some(id) => data_point.collection_id == id,
        })
        .map(|(i, _)| i)
        .collect();

    AggregatedDataPoints {
        sort_key,
        sort_direction,
        output_url: output_url.into(),
        page_size,
        data_points: list_items,
    }
}
