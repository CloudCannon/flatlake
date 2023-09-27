use std::cmp::Ordering;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Error;
use futures::future::join_all;
use serde_json::{Number, Value};
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

use crate::options::SortDirection;
use crate::{AggregateDataPoints, DataPoint, DataPointReference, LakeContext};

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
    agg: AggregateDataPoints,
    data: &Vec<DataPoint>,
    ctx: &LakeContext,
) -> Result<(), Error> {
    let mut content: Vec<_> = agg
        .lists
        .iter()
        .flat_map(|p| data.get(*p))
        .flat_map(|d| {
            d.front_matter.as_ref().map(|fm| DataPointReference {
                url: &d.output_url,
                data: fm,
            })
        })
        .collect();

    content.sort_by(|a, b| {
        let o = match (a.data.get(&agg.sort_key), b.data.get(&agg.sort_key)) {
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

    let content = match serde_json::to_vec_pretty(&content) {
        Ok(content) => content,
        Err(e) => {
            ctx.logger.error(format!(
                "Failed to parse content for {:?}: {}",
                agg.output_url, e,
            ));
            return Err(e.into());
        }
    };

    let disk_output_url = ctx.params.dest.join(agg.output_url);

    ctx.logger
        .v_info(format!("Writing file at {disk_output_url:?}"));

    write(disk_output_url, content).await;

    Ok(())
}

pub async fn create_output(data_point: DataPoint, ctx: &LakeContext) -> Result<(), Error> {
    let content = match serde_json::to_vec_pretty(&data_point.front_matter) {
        Ok(content) => content,
        Err(e) => {
            ctx.logger.error(format!(
                "Failed to parse content for {:?}: {}",
                data_point.output_url, e,
            ));
            return Err(e.into());
        }
    };

    let disk_output_url = ctx.params.dest.join(data_point.output_url);

    ctx.logger
        .v_info(format!("Writing file at {disk_output_url:?}"));

    write(disk_output_url, content).await;

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
