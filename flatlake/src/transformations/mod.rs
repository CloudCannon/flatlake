use markdown::ParseOptions;
use serde_json::json;

use crate::{
    options::{OutputElement, OutputMethod},
    DataPoint, LakeContext,
};

impl DataPoint {
    pub fn hydrate_data_point(&mut self, ctx: &LakeContext) {
        let collection_options = ctx
            .params
            .collections
            .get(self.collection_id)
            .expect("Data point should be assigned to a valid collection");

        let input_options = collection_options
            .inputs
            .get(self.input_id)
            .expect("Input should exist");

        let outputs = collection_options
            .outputs
            .as_ref()
            .unwrap_or(&ctx.params.global.outputs);

        if let Some(merge_data) = &input_options.merge_data {
            if let Some(serde_json::Value::Object(base_data)) = self.front_matter.as_mut() {
                base_data.extend(merge_data.clone().into_iter());
            }
        }

        if outputs.contains(&OutputMethod::Single) {
            self.output_as_single = true;
        }

        if self.content_ast.is_some() {
            return;
        }

        if let Some(content) = self.content.as_ref() {
            let mut all_output_elements = collection_options
                .single_elements
                .iter()
                .chain(collection_options.list_elements.iter());

            if all_output_elements.any(|el| matches!(el, OutputElement::ContentAst)) {
                // Some output of this data point requires an AST,
                // so we should generate and store that now.

                let mut content_ast = serde_json::to_value(
                    markdown::to_mdast(content, &ParseOptions::default()).unwrap(),
                )
                .unwrap();
                prettify_md_ast(&mut content_ast);

                self.content_ast = Some(content_ast);
            }
        }
    }

    pub fn get_value_for_single(&self, ctx: &LakeContext) -> serde_json::Value {
        let mut output_object = json!({});
        let output_map = output_object.as_object_mut().unwrap();

        let single_elements = &ctx
            .params
            .collections
            .get(self.collection_id)
            .expect("Data point should be assigned to a valid collection")
            .single_elements;

        self.add_elements_to_object(single_elements, output_map);

        output_object
    }

    pub fn get_value_for_list(&self, ctx: &LakeContext) -> serde_json::Value {
        let mut output_object = json!({});
        let output_map = output_object.as_object_mut().unwrap();

        let list_elements = &ctx
            .params
            .collections
            .get(self.collection_id)
            .expect("Data point should be assigned to a valid collection")
            .list_elements;

        self.add_elements_to_object(list_elements, output_map);

        output_map.insert(
            "url".into(),
            serde_json::to_value(&self.get_normalized_url())
                .expect("Output URL should be serializable"),
        );

        output_object
    }

    fn add_elements_to_object(
        &self,
        elements: &Vec<OutputElement>,
        object: &mut serde_json::Map<String, serde_json::Value>,
    ) {
        for element in elements {
            match element {
                crate::options::OutputElement::Data => {
                    let front_matter_data = if let Some(front_matter) = self.front_matter.as_ref() {
                        match serde_json::to_value(front_matter) {
                            Ok(data) => data,
                            Err(e) => {
                                eprintln!("Failed to serialize a file: {e}");
                                // TODO: Error handle.
                                std::process::exit(1);
                            }
                        }
                    } else {
                        // Insert empty data as a fallback, so that the data key always exists on these objects
                        json!({})
                    };

                    object.insert("data".into(), front_matter_data);
                }
                crate::options::OutputElement::FlatData => {
                    let front_matter_data = if let Some(front_matter) = self.front_matter.as_ref() {
                        match serde_json::to_value(front_matter) {
                            Ok(data) => data,
                            Err(e) => {
                                eprintln!("Failed to serialize a file: {e}");
                                // TODO: Error handle.
                                std::process::exit(1);
                            }
                        }
                    } else {
                        json!({})
                    };

                    for (key, value) in front_matter_data
                        .as_object()
                        .expect("Front matter should be an object")
                        .into_iter()
                    {
                        object.insert(key.clone(), value.clone());
                    }
                }
                crate::options::OutputElement::Content => {
                    let content_data = if let Some(content) = self.content.as_ref() {
                        serde_json::to_value(content).unwrap()
                    } else {
                        // Insert empty content as a fallback, so that the content key always exists on these objects
                        json!("")
                    };

                    object.insert("content".into(), content_data);
                }
                crate::options::OutputElement::ContentAst => {
                    let content_ast_data = if let Some(content_ast) = self.content_ast.as_ref() {
                        content_ast.clone()
                    } else {
                        // Insert empty content as a fallback, so that the content key always exists on these objects
                        json!({})
                    };

                    object.insert("content_ast".into(), content_ast_data);
                }
            }
        }
    }
}

fn prettify_md_ast(input: &mut serde_json::Value) {
    match input {
        serde_json::Value::Array(values) => {
            for value in values {
                prettify_md_ast(value);
            }
        }
        serde_json::Value::Object(value) => {
            value.remove("position");
            for kv in value.values_mut() {
                prettify_md_ast(kv);
            }
        }
        _ => {}
    }
}
