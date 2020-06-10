use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};
use yaserde_derive::{YaDeserialize, YaSerialize};
use super::transport::Validate;

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "Content")]
pub struct Content {
    #[yaserde(prefix = "content", rename = "content")]
    pub content: Option<String>,

    #[yaserde(prefix = "content", rename = "fieldName")]
    pub field_name: String,

    #[yaserde(prefix = "content", rename = "fileName")]
    pub file_name: Option<String>,

    #[yaserde(prefix = "content", rename = "path")]
    pub path: Option<String>,

    #[yaserde(prefix = "content", rename = "recordId")]
    pub record_id: i64,
}

impl Validate for Content {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "ArrayOfResponse")]
pub struct ArrayOfResponse {
    #[yaserde(prefix = "content", rename = "Response")]
    pub response: Vec<Response>,
}

impl Validate for ArrayOfResponse {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "ArrayOfContentField")]
pub struct ArrayOfContentField {
    #[yaserde(prefix = "content", rename = "ContentField")]
    pub content_field: Vec<ContentField>,
}

impl Validate for ArrayOfContentField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "ContentField")]
pub struct ContentField {
    #[yaserde(prefix = "content", rename = "boName")]
    pub bo_name: Option<String>,

    #[yaserde(prefix = "content", rename = "fieldLabel")]
    pub field_label: Option<String>,

    #[yaserde(prefix = "content", rename = "fieldName")]
    pub field_name: Option<String>,

    #[yaserde(prefix = "content", rename = "fieldType")]
    pub field_type: Option<String>,

    #[yaserde(prefix = "content", rename = "moduleName")]
    pub module_name: Option<String>,
}

impl Validate for ContentField {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "ArrayOfContent-1-")]
pub struct ArrayOfContent1 {
    #[yaserde(prefix = "content", rename = "Content")]
    pub content: Vec<Content>,
}

impl Validate for ArrayOfContent1 {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "InvalidContentException")]
pub struct InvalidContentException {}

impl Validate for InvalidContentException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "InvalidDocumentTypeException")]
pub struct InvalidDocumentTypeException {}

impl Validate for InvalidDocumentTypeException {}


#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "content", namespace = "content: http://content.dto.ws.tririga.com", rename = "Response")]
pub struct Response {
    #[yaserde(prefix = "content", rename = "checksum")]
    pub checksum: Option<i64>,

    #[yaserde(prefix = "content", rename = "length")]
    pub length: Option<i64>,

    #[yaserde(prefix = "content", rename = "message")]
    pub message: Option<String>,

    #[yaserde(prefix = "content", rename = "mimeType")]
    pub mime_type: Option<String>,

    #[yaserde(prefix = "content", rename = "status")]
    pub status: Option<String>,

    #[yaserde(prefix = "content", rename = "updatedDate")]
    pub updated_date: Option<String>,

    pub base: Content,
}

impl Validate for Response {}
