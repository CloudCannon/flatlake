use crate::DataPoint;
use crate::LakeContext;
use crate::Tributary;
use anyhow::anyhow;
use anyhow::Error;
use serde::Deserialize;

impl Tributary {
    pub async fn read_file(self, ctx: &LakeContext) -> Result<DataPoint, Error> {
        let Some(file_path) = &self.file_path else { todo!("Handle synthetic files") };

        let output_url = file_path
            .strip_prefix(&self.root_path.unwrap())
            .unwrap()
            .with_extension("json");

        let mut data_point = DataPoint {
            collection_id: self.collection_id,
            output_url,
            front_matter: None,
            content: None,
        };
        let mut front_matter_str: Option<String> = None;

        // Temporary file parsing logic

        let contents = tokio::fs::read_to_string(file_path).await?;
        let mut lines = contents.lines();

        while let Some(line) = lines.next() {
            if let Some(content) = data_point.content.as_mut() {
                content.push('\n');
                content.push_str(line);
            } else if front_matter_str.is_none() && line.trim() == "---" {
                front_matter_str = Some("".into());
            } else if front_matter_str.is_some() && line.trim() == "---" {
                match serde_yaml::from_str::<serde_json::Value>(&front_matter_str.take().unwrap()) {
                    Ok(fm) => data_point.front_matter = Some(fm),
                    Err(e) => {
                        ctx.logger.error(format!(
                            "{:?} errored when parsing front matter: {}",
                            file_path, e
                        ));
                        return Err(anyhow!("Unparseable"));
                    }
                }

                data_point.content = Some("".into());
            } else if let Some(front_matter_str) = front_matter_str.as_mut() {
                front_matter_str.push('\n');
                front_matter_str.push_str(line);
            }
        }

        Ok(data_point)
    }
}
