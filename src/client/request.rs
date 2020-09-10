use crate::client::{Client, Error, Response};
use chrono::prelude::*;
use crypto::digest::Digest;
use crypto::md5::Md5;
use serde;
use serde::de::DeserializeOwned;
use serde_json;
use serde_json::Result as JSONResult;
use std::collections::HashMap;

const API_VERSION: &str = "2.0";

pub trait Request: serde::Serialize
where
    Self: std::marker::Sized,
{
    type ResponseType: Response + DeserializeOwned;

    fn method(&self) -> String;

    fn to_json(&self) -> JSONResult<String> {
        serde_json::to_string(self)
    }

    fn is_union_gateway(&self) -> bool {
        return false;
    }

    fn is_log_gateway(&self) -> bool {
        return false;
    }

    fn sign(&self, clt: &Client, access_token: String) -> Result<HashMap<String, String>, Error> {
        let param = self.to_json()?;
        let mut keys = vec![
            ("method", self.method()),
            ("access_token", access_token),
            ("app_key", clt.app_key.to_string()),
            (
                "timestamp",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            ),
            ("format", String::from("json")),
        ];
        if self.is_union_gateway() {
            keys.push(("param_json", param));
            keys.push(("v", "1.0".to_string()));
        } else {
            keys.push(("360buy_param_json", param));
            keys.push(("v", API_VERSION.to_string()));
        }
        keys.sort_by(|a, b| b.0.cmp(&a.0));
        let joind: String = keys
            .iter()
            .map(|v| format!("{}={}", v.0.to_string(), v.1))
            .collect::<Vec<String>>()
            .concat();
        let to_be_signed: String = format!("{}{}{}", clt.secret, joind, clt.secret);
        let mut md5 = Md5::new();
        md5.input_str(&to_be_signed);
        keys.push(("sign", md5.result_str()));
        let mut mp: HashMap<String, String> = HashMap::with_capacity(8);
        for v in &keys {
            mp.insert(v.0.to_string(), v.1.clone());
        }
        Ok(mp)
    }

    fn run<T>(
        &self,
        clt: &Client,
        access_token: T,
    ) -> Result<<Self::ResponseType as Response>::ResultType, Error>
    where
        T: Into<String>,
    {
        return clt.run(self, access_token);
    }
}
