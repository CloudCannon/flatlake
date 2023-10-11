use std::cmp::Ordering;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Error;
use futures::future::join_all;
use serde::Serialize;
use serde_json::{json, Number, Value};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

use crate::options::SortDirection;
use crate::{AggregatedDataPoints, DataPoint, LakeContext};

#[derive(Serialize)]
struct PaginatedValues {
    page: usize,
    total_pages: usize,
    has_more: bool,
    next_page: Option<PathBuf>,
    values: Vec<Value>,
}

fn sort_number(x: &Number, y: &Number) -> Ordering {
    let x_f = x
        .as_f64()
        .or(x.as_i64().map(|n| n as f64))
        .or(x.as_u64().map(|n| n as f64));
    let y_f = y
        .as_f64()
        .or(y.as_i64().map(|n| n as f64))
        .or(y.as_u64().map(|n| n as f64));

    x_f.partial_cmp(&y_f).unwrap_or(Ordering::Equal)
}

pub async fn create_list_output(
    agg: AggregatedDataPoints,
    data: &Vec<DataPoint>,
    ctx: &LakeContext,
) -> Result<(), Error> {
    let mut output_list = agg.data_points.clone();
    output_list.sort_by(|a, b| {
        let a_key = data
            .get(*a)
            .map(|d| d.get_sort_value(&agg.sort_key))
            .flatten();
        let b_key = data
            .get(*b)
            .map(|d| d.get_sort_value(&agg.sort_key))
            .flatten();

        let o = match (a_key, b_key) {
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(Value::Number(x)), Some(Value::Number(y))) => sort_number(x, y),
            (Some(Value::String(x)), Some(Value::String(y))) => {
                x.partial_cmp(y).unwrap_or(Ordering::Equal)
            }
            _ => Ordering::Equal,
        };
        match agg.sort_direction {
            SortDirection::Asc => o,
            SortDirection::Desc => o.reverse(),
        }
    });

    let page_size = if agg.page_size == 0 {
        // A page size of 0 denotes all items on one page
        agg.data_points.len()
    } else {
        agg.page_size
    };

    let pages = output_list.chunks(page_size);
    let page_count = pages.len();

    let mut paged_output = pages
        .into_iter()
        .enumerate()
        .map(|(page_number, values)| {
            let logical_page = page_number + 1;
            let paged_output_url = agg.output_url.join(format!("page-{}.json", logical_page));
            return (paged_output_url, logical_page, values);
        })
        .peekable();

    while let Some((paged_output_url, page_number, values)) = paged_output.next() {
        let next_page = paged_output.peek();

        let output_data_points = values
            .into_iter()
            .map(|dp_index| {
                data.get(*dp_index)
                    .expect("Data point should exist")
                    .get_value_for_list(ctx)
            })
            .collect::<Vec<_>>();

        let output_page = PaginatedValues {
            page: page_number,
            total_pages: page_count,
            has_more: next_page.is_some(),
            next_page: next_page.map(|p| p.0.clone()),
            values: output_data_points,
        };

        let content = serde_json::to_vec_pretty(&output_page)
            .expect("Serializing PaginatedValues shouldn't fail");

        let disk_output_url = ctx.params.dest.join(paged_output_url);

        ctx.logger
            .v_info(format!("Writing file at {disk_output_url:?}"));

        write(disk_output_url, content).await;
    }

    Ok(())
}

pub async fn create_output(mut data_point: DataPoint, ctx: &LakeContext) -> Result<(), Error> {
    let output_value = data_point.get_value_for_single(ctx);
    let output_bytes =
        serde_json::to_vec_pretty(&output_value).expect("Serializing a Value shouldn't fail");

    let disk_output_url = ctx.params.dest.join(data_point.output_url);

    ctx.logger
        .v_info(format!("Writing file at {disk_output_url:?}"));

    write(disk_output_url, output_bytes).await;

    Ok(())
}

async fn write(filename: PathBuf, content_chunks: Vec<u8>) {
    if let Some(parent) = filename.parent() {
        create_dir_all(parent).await.unwrap();
    }

    let mut output_file = File::create(&filename).await;
    while output_file.is_err() {
        sleep(Duration::from_millis(100)).await;
        output_file = File::create(&filename).await;
    }

    if let Ok(mut file) = output_file {
        file.write_all(&content_chunks).await.unwrap();
    }
}
