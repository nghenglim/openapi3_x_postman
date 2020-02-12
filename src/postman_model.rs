use std::collections::BTreeMap;
use serde_json::value::Value;
use crate::model::{OpenApi3, OpenApi3Schema, OpenApi3ConvertOption, OpenApi3OperationMethodRequestBodyContent, OpenApi3OperationMethodRequestBodyJson, OpenApi3OperationMethodRequestBody, OpenApi3Parameter, OpenApi3SchemaStringType, OpenApi3SchemaObjectType, OpenApi3OperationMethodSecurity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollection2c1 {
    pub info: PostmanCollectionInfo,
    pub item: Vec<PostmanCollectionItem>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionInfo {
    pub name: String,
    pub description: Option<String>,
    pub schema: String,
}
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PostmanCollectionItemInner {
//     name: String,
//     request: PostmanCollectionRequest,
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionItem {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Box<Vec<PostmanCollectionItem>>>,
    pub request: Option<PostmanCollectionRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<Vec<PostmanCollectionResponse>>
    // request: PostmanCollectionRequest,
    // response: Vec<serde_json::Value>,
    // #[serde(rename = "protocolProfileBehavior")]
    // protocol_profile_behavior: serde_json::Value
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionResponse {
    pub status: Option<String>,
    pub code: Option<u32>,
    pub body: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<PostmanCollectionRequestAuth>,
    pub method: String,
    pub header: Vec<PostmanCollectionHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PostmanCollectionBody>,
    pub url: PostmanCollectionUrl,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionRequestAuth {
    #[serde(rename = "type")]
    _type: String,
    bearer: Option<Vec<PostmanCollectionRequestAuthItem>>,
    basic: Option<Vec<PostmanCollectionRequestAuthItem>>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionRequestAuthItem {
    key: String,
    value: String,
    #[serde(rename = "type")]
    _type: String,
}

impl PostmanCollectionRequest {
    pub fn oa3_get_securities(&self) -> OpenApi3OperationMethodSecurity {
        if let Some(auth) = &self.auth {
            let mut bmap = BTreeMap::new();
            if auth.bearer.is_some() {
                bmap.insert(auth._type.clone(), Vec::new());
            }
            Some(vec![bmap])
        } else {
            None
        }
    }
    pub fn oa3_get_request_body(&self) -> Option<OpenApi3OperationMethodRequestBody> {
        if let Some(body) = &self.body {
            Some(OpenApi3OperationMethodRequestBody {
                content: OpenApi3OperationMethodRequestBodyContent::ApplicationJson(OpenApi3OperationMethodRequestBodyJson {
                    schema: OpenApi3Schema::ObjectType(OpenApi3SchemaObjectType {
                        properties: BTreeMap::new(),
                        example: None,
                    }),
                    example: Some(Value::String(body.raw.clone())),
                })
            })
        } else {
            None
        }
    }
    pub fn oa3_get_parameters_for_header_query(&self, convert_option: &OpenApi3ConvertOption) -> Vec<OpenApi3Parameter> {
        let mut params: Vec<OpenApi3Parameter> = Vec::new();
        if let Some(urlquery) = &self.url.query {
            for query in urlquery {
                let example = match &query.value {
                    Value::String(s) => {
                        let mut val = Some(serde_json::value::Value::String(s.clone()));
                        for map in &convert_option.mapping {
                            if &map.key == s {
                                val = Some(serde_json::value::Value::String(map.value.clone()));
                            }
                        }
                        val
                    },
                    _ => None,
                };
                params.push(OpenApi3Parameter {
                    description: String::from(""),
                    _in: String::from("query"),
                    name: query.key.clone(),
                    required: false,
                    schema: OpenApi3Schema::StringType(OpenApi3SchemaStringType {
                        _type: String::from("string"),
                        _enum: None,
                        default: None,
                        example: example,
                    })
                });
            }
        }
        for headeritem in &self.header {
            let mut val = Some(serde_json::value::Value::String(headeritem.value.clone()));
            for map in &convert_option.mapping {
                if map.key == headeritem.key {
                    val = Some(serde_json::value::Value::String(map.value.clone()));
                }
            }
            params.push(OpenApi3Parameter {
                description: String::from(""),
                _in: String::from("header"),
                name: headeritem.key.clone(),
                required: false,
                schema: OpenApi3Schema::StringType(OpenApi3SchemaStringType {
                    _type: String::from("string"),
                    _enum: None,
                    default: None,
                    example: val,
                })
            });
        }
        params
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionHeader {
    key: String,
    #[serde(rename = "type")]
    _type: Option<String>,
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
    pub key: String,
    pub value: String,
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

impl PostmanCollectionUrl {
    pub fn path_extract(&self) -> (String, Vec<PostmanCollectionVariable>) {
        let mut paths: Vec<String> = Vec::new();
        let empty_vec = Vec::new();
        let thevars = if self.variable.is_some() {
            (self.variable).as_ref().unwrap()
        } else {
            &empty_vec
        };
        let mut vars: Vec<PostmanCollectionVariable> = Vec::new();
        // for host in &self.host {
        //     if host.starts_with(":") {
        //         let variable = (&host[1..host.len()]).to_owned();
        //         for v in thevars {
        //             if v.key == variable {
        //                 vars.push(v.clone());
        //             }
        //         }
        //         paths.push(variable);
        //     } else {
        //         paths.push(host.clone());
        //     }
        // }
        for path in &self.path {
            if path.starts_with(":") {
                let variable = (&path[1..path.len()]).to_owned();
                for v in thevars {
                    if v.key == variable {
                        vars.push(v.clone());
                    }
                }
                paths.push(format!("{{{}}}", variable));
            } else {
                paths.push(path.clone());
            }
        }
        (format!("/{}", paths.join("/")), vars)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanCollectionUrlQuery {
    key: String,
    value: serde_json::value::Value,
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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostmanConvertOption {
    pub host: String,
    pub preceding_path: String,
    pub prepend_tag: String,
    pub map_header: Vec<PostmanConvertOptionMapHeader>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostmanConvertOptionMapHeader {
    key: String,
    value: String,
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
                        value: serde_json::value::Value::String(example_value),
                    })
                } else if parameter._in == "header" {
                    let lc_name: String = parameter.name.clone().to_lowercase();
                    let mut the_value: Option<String> = Some(example_value);
                    for kv in postman_convert_option.map_header.clone() {
                        if kv.key.clone().to_lowercase() == lc_name {
                            the_value = Some(kv.value);
                            break;
                        }
                    }
                    header_vec.push(PostmanCollectionHeader {
                        key: parameter.name.clone(),
                        _type: Some("text".into()),
                        value: the_value.unwrap(),
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
                    paths[i] = format!("{}{}", ':', the_path[1..(the_path.len() - 1)].to_owned());
                }
            }
            let variable = if variable_vec.len() > 0 {
                Some(variable_vec)
            } else {
                None
            };
            let item_inner = PostmanCollectionItem {
                name: path.clone(),
                item: None,
                request: Some(PostmanCollectionRequest {
                    auth: None,
                    url: PostmanCollectionUrl {
                        raw: format!("{}/{}", postman_convert_option.host.clone(), paths.join("/")),
                        host: vec![postman_convert_option.host.clone()],
                        path: paths,
                        query: query,
                        variable: variable,
                    },
                    body: body,
                    header: header_vec,
                    description: None,
                    method: method.clone(),
                }),
                response: None,
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
                            request: None,
                            response: None,
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
            description: Some(openapi3.info.description.into()),
            schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json".into(),
        },
        item: item_base.into()
    }
}
