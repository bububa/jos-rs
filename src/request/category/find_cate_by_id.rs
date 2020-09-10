use crate::client::{APIError, Error, Request, Response, ResponseError};
use crate::request::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FindCateByIdRequest {
    #[serde(rename = "cid")]
    cid: u64,
    #[serde(rename = "field", skip_serializing_if = "String::is_empty")]
    fields: String,
}

impl FindCateByIdRequest {
    pub fn new<T>(cid: u64, fields: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            cid: cid,
            fields: fields.into(),
        }
    }

    pub fn set_cid(&mut self, v: u64) -> &mut Self {
        self.cid = v;
        self
    }

    pub fn set_fields<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.fields = v.into();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindCateByIdResponse {
    #[serde(rename = "error_response")]
    error: Option<ResponseError>,
    #[serde(rename = "jingdong_category_read_findById_responce")]
    data: Option<FindCateByIdResult>,
}

impl Default for FindCateByIdResponse {
    fn default() -> Self {
        Self {
            error: None,
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindCateByIdResult {
    #[serde(rename = "category")]
    pub category: Category,
}

impl Request for FindCateByIdRequest {
    type ResponseType = FindCateByIdResponse;

    fn method(&self) -> String {
        String::from("jingdong.category.read.findById")
    }
}

impl Response for FindCateByIdResponse {
    type ResultType = Category;

    fn result(&mut self) -> Result<Self::ResultType, Error> {
        if let Some(err) = self.error.take() {
            return Err(Error::from(err));
        }
        if let Some(data) = self.data.take() {
            return Ok(data.category);
        }
        Err(Error::from(APIError::new(0, "no data")))
    }
}
