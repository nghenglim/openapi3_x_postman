use std::collections::BTreeMap;
use serde_json::value::Value;
use crate::model::{OpenApi3, OpenApi3Schema, OpenApi3OperationMethodRequestBodyContent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollection2c1 {
    info: PostmanCollectionInfo,
    item: Vec<PostmanCollectionItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionInfo {
    name: String,
    description: String,
    schema: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionItemInner {
    name: String,
    request: PostmanCollectionRequest,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionItem {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    item: Option<Box<Vec<PostmanCollectionItemInner>>>,
    // request: PostmanCollectionRequest,
    // response: Vec<serde_json::Value>,
    // #[serde(rename = "protocolProfileBehavior")]
    // protocol_profile_behavior: serde_json::Value
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionRequest {
    method: String,
    header: Vec<PostmanCollectionHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<PostmanCollectionBody>,
    url: PostmanCollectionUrl,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionHeader {
    key: String,
    #[serde(rename = "type")]
    _type: String,
    value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionBody {
    mode: String,
    raw: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<PostmanCollectionBodyOptions>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionBodyOptions {
    raw: PostmanCollectionBodyOptionsRaw
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionBodyOptionsRaw {
    language: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionVariable {
    key: String,
    value: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionUrl {
    raw: String,
    host: Vec<String>,
    path: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<Vec<PostmanCollectionUrlQuery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable: Option<Vec<PostmanCollectionVariable>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionUrlQuery {
    key: String,
    value: String,
}
fn get_schema_to_string(schema: &OpenApi3Schema) -> String {
    let example_value = match schema {
        OpenApi3Schema::ArrayType(s) => {s.example.clone()},
        OpenApi3Schema::ObjectType(s) => {s.example.clone()},
        OpenApi3Schema::StringType(s) => {s.example.clone()},
        OpenApi3Schema::IntegerType(s) => {s.example.clone()},
        OpenApi3Schema::BooleanType(s) => {s.example.clone()},
    };
    get_value_to_string(example_value)
}
fn get_value_to_string(example_value: Option<Value>) -> String {
    match example_value {
        Some(v) => match v {
            Value::String(ss) => ss,
            _ => v.to_string(),
        },
        None => "".to_owned(),
    }
}
pub struct PostmanConvertOption {
    pub host: String,
    pub preceding_path: String,
    pub prepend_tag: String,
}
pub fn to_postman_colletion_2c1(openapi3: OpenApi3, postman_convert_option: PostmanConvertOption) -> PostmanCollection2c1 {
    let mut item_base: Vec<PostmanCollectionItem> = Vec::new();
    let mut postman_collection_item_map: BTreeMap<String, PostmanCollectionItem> = BTreeMap::new();
    for (rawpath, operations) in &openapi3.paths {
        let path: String = format!("{}{}", postman_convert_option.preceding_path, rawpath);
        for (method, operation) in operations {
            let tag = if operation.tags.len() > 0 {
                operation.tags[0].clone()
            } else {
                "_base".to_owned()
            };
            let nonabsolute_path = path.trim_start_matches('/');
            let mut header_vec: Vec<PostmanCollectionHeader> = Vec::new();
            let mut query_vec: Vec<PostmanCollectionUrlQuery> = Vec::new();
            let mut variable_vec: Vec<PostmanCollectionVariable> = Vec::new();
            for parameter in &operation.parameters {
                let example_value = get_schema_to_string(&parameter.schema);
                if parameter._in == "query" {
                    query_vec.push(PostmanCollectionUrlQuery {
                        key: parameter.name.clone(),
                        value: example_value,
                    })
                } else if parameter._in == "header" {
                    header_vec.push(PostmanCollectionHeader {
                        key: parameter.name.clone(),
                        _type: "text".into(),
                        value: example_value,
                    })
                } else if parameter._in == "path" {
                    variable_vec.push(PostmanCollectionVariable {
                        key: parameter.name.clone(),
                        value: example_value,
                    })
                }
            }
            let query = if query_vec.len() > 0 {
                Some(query_vec)
            } else {
                None
            };
            let body: Option<PostmanCollectionBody> = if operation.request_body.is_some() {
                let raw: String = match operation.request_body.clone().unwrap().content {
                    OpenApi3OperationMethodRequestBodyContent::ApplicationJson(a) => {
                        get_value_to_string(a.example)
                    },
                };
                Some(PostmanCollectionBody {
                    mode: "raw".into(),
                    raw: raw.into(),
                    options: Some(PostmanCollectionBodyOptions {
                        raw: PostmanCollectionBodyOptionsRaw {
                            language: "json".into(),
                        }
                    })
                })
            } else {
                None
            };
            let mut paths: Vec<String> = nonabsolute_path.clone().split('/').map(|s| s.to_owned()).collect();
            for i in 0..paths.len() {
                let the_path = paths[i].clone();
                if paths[i].starts_with("{") {
                    paths[i] = the_path[1..(the_path.len() - 1)].to_owned();
                }
            }
            let variable = if variable_vec.len() > 0 {
                Some(variable_vec)
            } else {
                None
            };
            let item_inner = PostmanCollectionItemInner {
                name: path.clone(),
                request: PostmanCollectionRequest {
                    url: PostmanCollectionUrl {
                        raw: format!("{}/{}", postman_convert_option.host.clone(), nonabsolute_path),
                        host: vec![postman_convert_option.host.clone()],
                        path: nonabsolute_path.clone().split('/').map(|s| s.to_owned()).collect(),
                        query: query,
                        variable: variable,
                    },
                    body: body,
                    header: header_vec,
                    method: method.clone(),
                }
            };
            match postman_collection_item_map.get_mut(&tag) {
                Some(collection) => {
                    if let Some(ref mut bv) = collection.item {
                        bv.as_mut().push(item_inner);
                    }
                },
                None => {
                    postman_collection_item_map.insert(
                        tag.clone(),
                        PostmanCollectionItem {
                            name: format!("{}{}", postman_convert_option.prepend_tag.clone(), tag),
                            item: Some(Box::new(vec![item_inner])),
                        },
                    );
                }
            };
        }
    }
    for (_tag, item) in &postman_collection_item_map {
        item_base.push(item.clone());
    }
    PostmanCollection2c1 {
        info: PostmanCollectionInfo {
            name: openapi3.info.title.into(),
            description: openapi3.info.description.into(),
            schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json".into(),
        },
        item: item_base.into()
    }
}
