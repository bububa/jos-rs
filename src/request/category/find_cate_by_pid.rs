use crate::client::{APIError, Error, Request, Response, ResponseError};
use crate::request::category::Category;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FindCateByPidRequest {
    #[serde(rename = "parentCid")]
    pid: u64,
    #[serde(rename = "field", skip_serializing_if = "String::is_empty")]
    fields: String,
}

impl FindCateByPidRequest {
    pub fn new<T>(pid: u64, fields: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            pid: pid,
            fields: fields.into(),
        }
    }

    pub fn set_pid(&mut self, v: u64) -> &mut Self {
        self.pid = v;
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
pub struct FindCateByPidResponse {
    #[serde(rename = "error_response")]
    error: Option<ResponseError>,
    #[serde(rename = "jingdong_category_read_findByPId_responce")]
    data: Option<Categories>,
}

impl Default for FindCateByPidResponse {
    fn default() -> Self {
        Self {
            error: None,
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Categories {
    #[serde(rename = "categories")]
    pub categories: Vec<Category>,
}

impl Request for FindCateByPidRequest {
    type ResponseType = FindCateByPidResponse;

    fn method(&self) -> String {
        String::from("jingdong.category.read.findByPId")
    }
}

impl Response for FindCateByPidResponse {
    type ResultType = Vec<Category>;

    fn result(&mut self) -> Result<Self::ResultType, Error> {
        if let Some(err) = self.error.take() {
            return Err(Error::from(err));
        }
        if let Some(data) = self.data.take() {
            return Ok(data.categories);
        }
        Err(Error::from(APIError::new(0, "no data")))
    }
}
