use std::collections::BTreeMap;
use serde_json::Value;
use crate::postman_model::{PostmanCollection2c1, PostmanCollectionItem};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3 {
    pub info: OpenApi3Info,
    pub openapi: String,
    pub paths: BTreeMap<String, OpenApi3Operations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<OpenApi3Components>,
    pub servers: Vec<OpenApi3Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3Server {
    description: String,
    url: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApi3ConvertOption {
    pub servers: Vec<OpenApi3Server>,
    pub mapping: Vec<OpenApi3ConvertMap>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3ConvertMap {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3Components {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "securitySchemes")]
    pub security_schemes: Option<BTreeMap<String, OpenApi3ComponentsSecuritySchemesItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3ComponentsSecuritySchemesItem {
    #[serde(rename = "in")]
    pub _in: String,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3Info {
    pub description: String,
    pub title: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3Parameter {
    pub description: String,
    #[serde(rename = "in")]
    pub _in: String,
    pub name: String,
    pub required: bool,
    pub schema: OpenApi3Schema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3SchemaArrayType {
    pub items: Box<OpenApi3Schema>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3SchemaObjectType {
    pub properties: BTreeMap<String, Box<OpenApi3Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3SchemaStringType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "enum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _enum: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3SchemaIntegerType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3SchemaBooleanType {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3SchemaRefType {
    #[serde(rename = "$ref")]
    pub _ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenApi3Schema {
    ArrayType(OpenApi3SchemaArrayType),
    ObjectType(OpenApi3SchemaObjectType),
    StringType(OpenApi3SchemaStringType),
    IntegerType(OpenApi3SchemaIntegerType),
    BooleanType(OpenApi3SchemaBooleanType),
}

pub type OpenApi3OperationMethodSecurity = Option<Vec<BTreeMap<String, Vec<String>>>>;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3OperationMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    pub parameters: Vec<OpenApi3Parameter>,
    pub responses: OpenApi3Responses,
    pub tags: Vec<String>,
    #[serde(rename = "requestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<OpenApi3OperationMethodRequestBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: OpenApi3OperationMethodSecurity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3OperationMethodRequestBody {
    pub content: OpenApi3OperationMethodRequestBodyContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3OperationMethodRequestBodyJson {
    pub schema: OpenApi3Schema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OpenApi3OperationMethodRequestBodyContent {
    #[serde(rename = "application/json")]
    ApplicationJson(OpenApi3OperationMethodRequestBodyJson)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<OpenApi3ResponseContents>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3ResponseContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<OpenApi3Schema>,
}

pub type OpenApi3ResponseContents = BTreeMap<String, OpenApi3ResponseContent>;

pub type OpenApi3Operations = BTreeMap<String, OpenApi3OperationMethod>;

pub type OpenApi3Responses = BTreeMap<String, OpenApi3Response>;

fn pmanitem_to_oa3ops(paths: &mut BTreeMap<String, OpenApi3Operations>, pmanitem: &PostmanCollectionItem, tag: &Option<String>, convert_option: &OpenApi3ConvertOption) {
    let response_empty_vec = Vec::new();
    let pman_responses = if pmanitem.response.is_some() {
        pmanitem.response.as_ref().unwrap()
    } else {
        &response_empty_vec
    };
    if let Some(pmanrequest) = &pmanitem.request {
        let mut tags = Vec::new();
        if let Some(tagstr) = tag {
            tags.push(tagstr.to_owned());
        }
        let mut parameters = pmanrequest.oa3_get_parameters_for_header_query(&convert_option);
        let (mut path, variables) = pmanrequest.url.path_extract();
        for variable in variables {
            parameters.push(OpenApi3Parameter {
                description: String::from(""),
                _in: String::from("path"),
                name: variable.key.clone(),
                required: true,
                schema: OpenApi3Schema::StringType(OpenApi3SchemaStringType {
                    _type: String::from("string"),
                    _enum: None,
                    default: None,
                    example: Some(Value::String(variable.value.clone()))
                })
            })
        }
        for mapp in &convert_option.mapping {
            path = str::replace(path.clone().as_ref(), &mapp.key, &mapp.value);
        }
        let method = pmanrequest.method.to_ascii_lowercase();
        let inner = paths.get_mut(&path);
        let mut responses: OpenApi3Responses = BTreeMap::new();
        for resp in pman_responses {
            if resp.code.is_some() {
                let mut response_content: OpenApi3ResponseContents = BTreeMap::new();
                response_content.insert(String::from("application/json"), OpenApi3ResponseContent {
                    description: None,
                    // schema: Some(OpenApi3Schema::RefType(OpenApi3SchemaRefType {
                    //     _ref: Some(String::from("#/components/schemas/Any"))
                    // })),
                    schema: Some(OpenApi3Schema::ObjectType(OpenApi3SchemaObjectType {
                        properties: BTreeMap::new(),
                        example: Some(Value::String(resp.body.clone()))
                    })),
                });
                responses.insert(format!("{}", resp.code.as_ref().unwrap()), OpenApi3Response {
                    description: Some(resp.status.as_ref().unwrap().clone()),
                    content: Some(response_content),
                });
            }
        }
        if responses.len() == 0 {
            let mut response_content: OpenApi3ResponseContents = BTreeMap::new();
            response_content.insert(String::from("application/json"), OpenApi3ResponseContent {
                description: None,
                // schema: Some(OpenApi3Schema::RefType(OpenApi3SchemaRefType {
                //     _ref: Some(String::from("#/components/schemas/Any"))
                // })),
                schema: Some(OpenApi3Schema::ObjectType(OpenApi3SchemaObjectType {
                    properties: BTreeMap::new(),
                    example: None
                })),
            });
            responses.insert(String::from("200"), OpenApi3Response {
                description: Some(String::from("OK")),
                content: Some(response_content),
            });
        }
        let oa3opmethod = OpenApi3OperationMethod {
            description: pmanrequest.description.clone(),
            operation_id: None,
            parameters: parameters,
            responses: responses,
            tags: tags,
            security: pmanrequest.oa3_get_securities(),
            request_body: pmanrequest.oa3_get_request_body(),
        };
        if let Some(innerbmap) = inner {
            innerbmap.insert(method, oa3opmethod);
        } else {
            let mut bmap = BTreeMap::new();
            bmap.insert(method, oa3opmethod);
            paths.insert(path, bmap);
        }
    }
    if let Some(inneritem) = &pmanitem.item {
        for pmaniteminner in inneritem.as_ref() {
            pmanitem_to_oa3ops(paths, pmaniteminner, tag, &convert_option)
        }
    }
}
pub fn to_openapi3(pman: PostmanCollection2c1, convert_option: OpenApi3ConvertOption) -> OpenApi3 {
    let mut paths: BTreeMap<String, OpenApi3Operations> = BTreeMap::new();

    for pmanitem in pman.item {
        pmanitem_to_oa3ops(&mut paths, &pmanitem, &Some(pmanitem.name.clone()), &convert_option)
    }
    let mut security_schemes = BTreeMap::new();
    security_schemes.insert("bearer".to_owned(), OpenApi3ComponentsSecuritySchemesItem {
        _in: String::from("header"),
        name: String::from("Authorization"),
        _type: String::from("apiKey"),
    });
    OpenApi3 {
        info: OpenApi3Info {
            description: String::from(""),
            title: pman.info.name.clone(),
            version: String::from("0.1.0"),
        },
        openapi: String::from("3.0.0"),
        paths: paths,
        components: Some(OpenApi3Components {
            security_schemes: Some(security_schemes),
        }),
        servers: convert_option.servers
    }
}
