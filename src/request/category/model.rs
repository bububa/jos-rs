use num::Zero;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    #[serde(rename = "fid", skip_serializing_if = "Zero::is_zero")]
    pub fid: u64, // 类目父ID
    #[serde(rename = "id", skip_serializing_if = "Zero::is_zero")]
    pub id: u64, // 类目id
    #[serde(rename = "lev", skip_serializing_if = "Zero::is_zero")]
    pub lev: u16, // 类目级别
    #[serde(rename = "name", skip_serializing_if = "String::is_empty")]
    pub name: String, // 类目名称
    #[serde(rename = "order", skip_serializing_if = "Zero::is_zero")]
    pub order: i16, // 排序
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<Feature>>, // 类目特殊属性列表
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    #[serde(rename = "featureKey", skip_serializing_if = "String::is_empty")]
    pub key: String, // 特殊属性key
    #[serde(rename = "featureValue", skip_serializing_if = "String::is_empty")]
    pub value: String, // 特殊属性value
    #[serde(rename = "featureCn", skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>, // 特殊属性中文含义
}
