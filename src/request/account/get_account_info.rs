use crate::client::{APIError, Error, Request, Response, ResponseError};
use num::Zero;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GetAccountInfoRequest {
    #[serde(rename = "accountType")]
    account_type: u16,
    #[serde(rename = "accountCode")]
    account_code: String,
}

impl GetAccountInfoRequest {
    pub fn new<T>(account_type: u16, account_code: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            account_type: account_type,
            account_code: account_code.into(),
        }
    }

    pub fn set_account_type(&mut self, v: u16) -> &mut Self {
        self.account_type = v;
        self
    }

    pub fn set_account_code<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.account_code = v.into();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAccountInfoResponse {
    #[serde(rename = "error_response")]
    error: Option<ResponseError>,
    #[serde(rename = "ingdong_pop_account_getAccountInfo_responce")]
    data: Option<AccountInfoResponse>,
}

impl Default for GetAccountInfoResponse {
    fn default() -> Self {
        Self {
            error: None,
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfoResponse {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub code: String,
    #[serde(rename = "errorMessage", skip_serializing_if = "String::is_empty")]
    pub error_message: String,
    #[serde(rename = "errorSolution", skip_serializing_if = "String::is_empty")]
    pub error_solution: String,
    #[serde(rename = "bean_account")]
    pub bean_account: Option<BeanAccount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BeanAccount {
    #[serde(rename = "accountId", skip_serializing_if = "Zero::is_zero")]
    pub account_id: u64,
    #[serde(skip_serializing_if = "Zero::is_zero")]
    pub status: u16,
    #[serde(rename = "mainNum", skip_serializing_if = "Zero::is_zero")]
    pub main_num: u32, // 充值（主）账户京豆数量
    #[serde(rename = "subNum", skip_serializing_if = "Zero::is_zero")]
    pub sub_num: u32, // 充值（副）账户京豆数量
    #[serde(skip_serializing_if = "String::is_empty")]
    pub creator: String, // 创建人
    #[serde(skip_serializing_if = "Zero::is_zero")]
    pub modified: i64, // 最近一次修改时间
    #[serde(rename = "accountCode", skip_serializing_if = "String::is_empty")]
    pub account_code: String, // 京豆账户,当accountType=1，则accountCode为商家id;当accountType=2，则accouontCode为供应商简码
    #[serde(skip_serializing_if = "Zero::is_zero")]
    pub created: i64, // 创建时间
    #[serde(rename = "freezeNum", skip_serializing_if = "Zero::is_zero")]
    pub freeze_num: u32, // 冻结京豆数量
    #[serde(rename = "accountType", skip_serializing_if = "Zero::is_zero")]
    pub account_type: u16, // 京豆账户类型,1:商家账户,2:供应商账户,3:品牌商账户
    #[serde(rename = "availableNum", skip_serializing_if = "Zero::is_zero")]
    pub available_num: u32, // 可用京豆数量
}

impl Request for GetAccountInfoRequest {
    type ResponseType = GetAccountInfoResponse;

    fn method(&self) -> String {
        String::from("jingdong.pop.account.getAccountInfo")
    }
}

impl Response for GetAccountInfoResponse {
    type ResultType = AccountInfoResponse;

    fn result(&mut self) -> Result<Self::ResultType, Error> {
        if let Some(err) = self.error.take() {
            return Err(Error::from(err));
        }
        if let Some(data) = self.data.take() {
            return Ok(data);
        }
        Err(Error::from(APIError::new(0, "no data")))
    }
}
