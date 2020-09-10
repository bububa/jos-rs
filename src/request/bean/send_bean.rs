use crate::client::{APIError, Error, Request, Response, ResponseError};
use num::Zero;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SendBeanRequest {
    #[serde(rename = "requestId")]
    request_id: String, // 防重入ID
    #[serde(rename = "beanNum")]
    bean_num: u32, // 发豆数量
    #[serde(rename = "accountId", skip_serializing_if = "Zero::is_zero")]
    account_id: u64, // 京豆账户ID
    #[serde(rename = "accountCode")]
    account_code: String, // 京豆账户,当accountType=1，则accountCode为商家id;当accountType=2，则accouontCode为供应商简码
    #[serde(rename = "sendWay")]
    send_way: u16, // 发豆方式,目前只支持按PIN发豆，固定值为1
    #[serde(rename = "sendCode")]
    send_code: String, // 发豆参数,当sendWay=1, 为用户pin,目前只支持按PIN发豆，固定值为1
    #[serde(rename = "accountType")]
    account_type: u16, // 京豆账户类型,1:商家账户,2:供应商账户,3:品牌商账户
    #[serde(rename = "context")]
    context: String, // 发豆说明
    #[serde(rename = "planId")]
    plan_id: u64, // 计划ID
    #[serde(rename = "rfld", skip_serializing_if = "String::is_empty")]
    rfld: String, // 外部活动id
}

impl SendBeanRequest {
    pub fn set_request_id<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.request_id = v.into();
        self
    }

    pub fn set_bean_num(&mut self, v: u32) -> &mut Self {
        self.bean_num = v;
        self
    }

    pub fn set_account_id(&mut self, v: u64) -> &mut Self {
        self.account_id = v;
        self
    }

    pub fn set_account_code<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.account_code = v.into();
        self
    }

    pub fn set_send_way(&mut self, v: u16) -> &mut Self {
        self.send_way = v;
        self
    }

    pub fn set_send_code<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.send_code = v.into();
        self
    }

    pub fn set_account_type(&mut self, v: u16) -> &mut Self {
        self.account_type = v;
        self
    }

    pub fn set_context<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.context = v.into();
        self
    }

    pub fn set_plan_id(&mut self, v: u64) -> &mut Self {
        self.plan_id = v;
        self
    }

    pub fn set_rfld<T>(&mut self, v: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.rfld = v.into();
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendBeanResponse {
    #[serde(rename = "error_response")]
    error: Option<ResponseError>,
    #[serde(rename = "jingdong_pop_bean_sendBean_responce")]
    data: Option<SendBeanResult>,
}

impl Default for SendBeanResponse {
    fn default() -> Self {
        Self {
            error: None,
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendBeanResult {
    #[serde(rename = "result")]
    pub result: u32,
    #[serde(rename = "remainBeanNum")]
    pub remain_bean_num: u64,
    #[serde(rename = "desc")]
    pub desc: String,
}

impl Request for SendBeanRequest {
    type ResponseType = SendBeanResponse;

    fn method(&self) -> String {
        String::from("jingdong.pop.bean.sendBean")
    }
}

impl Response for SendBeanResponse {
    type ResultType = SendBeanResult;

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
