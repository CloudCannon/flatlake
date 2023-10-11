use el_slugify::slugify;
use std::collections::{HashMap, HashSet};

use crate::{options::SortDirection, AggregatedDataPoints, DataPoint, LakeContext};

pub async fn generate_all_aggregate_files(
    ctx: &LakeContext,
    data: &Vec<DataPoint>,
) -> Vec<AggregatedDataPoints> {
    let mut all_aggs = vec![];

    all_aggs.extend(generate_aggregate_files(ctx, data, None).await);

    for i in 0..ctx.params.collections.len() {
        all_aggs.extend(generate_aggregate_files(ctx, data, Some(i)).await);
    }

    all_aggs
}

pub async fn generate_aggregate_files(
    ctx: &LakeContext,
    data: &Vec<DataPoint>,
    for_collection_id: Option<usize>,
) -> Vec<AggregatedDataPoints> {
    let mut aggregate_data = vec![];

    let page_size = for_collection_id
        .map(|i| {
            ctx.params
                .collections
                .get(i)
                .expect("Aggregate should match a valid collection")
                .page_size
        })
        .flatten()
        .unwrap_or(ctx.params.global.page_size);

    let mut front_matter_keys: HashMap<&String, HashMap<String, Vec<usize>>> = HashMap::new();

    // Loop 1: Collect all aggregable key/values
    for dp in data
        .iter()
        .filter(|data| match for_collection_id {
            None => true,
            Some(id) => data.collection_id == id,
        })
        .flat_map(|data| data.front_matter.as_ref().map(|fm| fm.as_object()))
        .flatten()
    {
        for (key, value) in dp.iter() {
            let keystore = match front_matter_keys.get_mut(key) {
                Some(ks) => ks,
                None => {
                    front_matter_keys.insert(key, HashMap::new());
                    front_matter_keys.get_mut(key).unwrap()
                }
            };
            match value {
                serde_json::Value::String(s) => {
                    keystore.insert(s.clone(), vec![]);
                }
                serde_json::Value::Array(a) => {
                    a.iter()
                        .filter(|v| matches!(v, serde_json::Value::String(_)))
                        .for_each(|s| {
                            keystore.insert(s.as_str().unwrap().to_owned(), vec![]);
                        });
                }
                _ => { /* Other types are not aggregated */ }
            };
        }
    }

    // Loop 2: House data points in each aggregable location
    for (data_point_index, data) in data
        .iter()
        .enumerate()
        .filter(|(_, data)| match for_collection_id {
            None => true,
            Some(id) => data.collection_id == id,
        })
        .flat_map(|(data_point_index, data)| {
            data.front_matter
                .as_ref()
                .map(|fm| fm.as_object().map(|o| (data_point_index, o)))
        })
        .flatten()
    {
        for (key, value) in data.iter() {
            let Some(keystore) = front_matter_keys.get_mut(key) else {
                 continue;
            };

            let mut add_to_agg = |loc_str: &String| {
                let Some(valuestore) = keystore.get_mut(loc_str) else {
                    return;
                };

                valuestore.push(data_point_index);
            };

            match value {
                serde_json::Value::String(s) => {
                    add_to_agg(s);
                }
                serde_json::Value::Array(a) => {
                    a.iter()
                        .filter(|v| matches!(v, serde_json::Value::String(_)))
                        .for_each(|s| add_to_agg(&s.as_str().unwrap().to_owned()));
                }
                _ => { /* Other types are not aggregated */ }
            };
        }
    }

    for (key, entries) in front_matter_keys.into_iter() {
        let output_key = slugify(key);
        for (value, pages) in entries.into_iter() {
            let output_value = slugify(&value);
            let output_url = match for_collection_id {
                Some(id) => {
                    let collection_key = &ctx
                        .params
                        .collections
                        .get(id)
                        .expect("Collection ID should exist")
                        .output_key;
                    format!("{collection_key}/aggregate/{output_key}/{output_value}/")
                }
                None => format!("aggregate/{output_key}/{output_value}/"),
            };

            let (sort_key, sort_direction) = match for_collection_id {
                Some(id) => {
                    let coll = &ctx
                        .params
                        .collections
                        .get(id)
                        .expect("Collection ID should exist");
                    (coll.sort_key.clone(), coll.sort_direction)
                }
                None => (
                    ctx.params.global.sort_key.clone(),
                    ctx.params.global.sort_direction,
                ),
            };

            aggregate_data.push(AggregatedDataPoints {
                sort_key,
                sort_direction,
                output_url: output_url.into(),
                page_size,
                data_points: pages,
            });
        }
    }

    aggregate_data
}
