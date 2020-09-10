use crate::client::{APIError, Error, Request, Response, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct QueryBrandsIdByVenderIdRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBrandsIdByVenderIdResponse {
    #[serde(rename = "error_response")]
    error: Option<ResponseError>,
    #[serde(rename = "jingdong_pop_brand_client_queryBrandsIdByVenderId_response")]
    data: Option<QueryBrandsIdByVenderIdResult>,
}

impl Default for QueryBrandsIdByVenderIdResponse {
    fn default() -> Self {
        Self {
            error: None,
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct QueryBrandsIdByVenderIdResult {
    #[serde(rename = "result")]
    pub result: BrandsId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrandsId {
    #[serde(rename = "brandsId")]
    pub id: u64,
}

impl Request for QueryBrandsIdByVenderIdRequest {
    type ResponseType = QueryBrandsIdByVenderIdResponse;

    fn method(&self) -> String {
        String::from("jingdong.pop.brand.client.queryBrandsIdByVenderId")
    }
}

impl Response for QueryBrandsIdByVenderIdResponse {
    type ResultType = u64;

    fn result(&mut self) -> Result<Self::ResultType, Error> {
        if let Some(err) = self.error.take() {
            return Err(Error::from(err));
        }
        if let Some(data) = self.data.take() {
            return Ok(data.result.id);
        }
        Err(Error::from(APIError::new(0, "no data")))
    }
}
