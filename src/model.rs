use std::collections::BTreeMap;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi3 {
    pub info: OpenApi3Info,
    pub openapi: String,
    pub paths: BTreeMap<String, OpenApi3Operations>
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
#[serde(untagged)]
pub enum OpenApi3Schema {
    ArrayType(OpenApi3SchemaArrayType),
    ObjectType(OpenApi3SchemaObjectType),
    StringType(OpenApi3SchemaStringType),
    IntegerType(OpenApi3SchemaIntegerType),
    BooleanType(OpenApi3SchemaBooleanType),
}
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
    pub schema: OpenApi3Schema
}

pub type OpenApi3ResponseContents = BTreeMap<String, OpenApi3ResponseContent>;

pub type OpenApi3Operations = BTreeMap<String, OpenApi3OperationMethod>;

pub type OpenApi3Responses = BTreeMap<String, OpenApi3Response>;
