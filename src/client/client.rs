use crate::client::Error;
use crate::client::{Request, Response};

use reqwest;
use serde_json;
use std::borrow::Cow;
use std::collections::HashMap;
#[warn(unused_variables)]
const GATEWAY_URL: &str = "https://api.jd.com/routerjson";
#[warn(unused_variables)]
const LOG_GATEWAY_URL: &str = "https://api-log.jd.com/routerjson";
#[warn(unused_variables)]
const UNION_GATEWAY_URL: &str = "https://router.jd.com/api";

pub struct Client<'a> {
    pub(crate) app_key: Cow<'a, str>,
    pub(crate) secret: Cow<'a, str>,
    pub(crate) debug: bool,
}

impl Client<'_> {
    pub fn new(app_key: &str, secret: &str) -> Self {
        Self {
            app_key: Cow::Owned(app_key.to_string()),
            secret: Cow::Owned(secret.to_string()),
            debug: false,
        }
    }

    pub fn set_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    pub fn run<'a, T, S>(
        &self,
        req: &T,
        access_token: S,
    ) -> Result<<T::ResponseType as Response>::ResultType, Error>
    where
        T: Request,
        S: Into<String>,
    {
        let params: HashMap<String, String> = req.sign(&self, access_token.into())?;
        let payload: String = serde_json::to_string(&params)?;
        let gateway: &str;
        if req.is_union_gateway() {
            gateway = UNION_GATEWAY_URL;
        } else if req.is_log_gateway() {
            gateway = LOG_GATEWAY_URL;
        } else {
            gateway = GATEWAY_URL;
        }
        println!("gateway: {}, payload: {}", gateway, payload);
        let http_client = reqwest::blocking::Client::new();
        let ret = http_client.get(gateway).query(&params).send()?.text()?;
        println!("ret: {}", ret);
        let mut resp: T::ResponseType = http_client.get(gateway).query(&params).send()?.json()?;
        resp.result()
    }
}
