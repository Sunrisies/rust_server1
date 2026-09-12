use serde::{Deserialize, Serialize};

/// 船只功能实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFunction {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub function_type: Option<String>,
    pub code: Option<String>,
    pub remark: Option<String>,
    #[serde(rename = "on")]
    pub is_on: Option<String>,
    // 查询参数
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub keywords: Option<String>,
    pub taskid: Option<String>,
}

impl NodeFunction {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            function_type: None,
            code: None,
            remark: None,
            is_on: Some("1".to_string()),
            page: None,
            limit: None,
            keywords: None,
            taskid: None,
        }
    }
}

/// 船只功能关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFunAndNode {
    pub id: Option<String>,
    pub nid: Option<String>,
    pub fid: Option<String>,
}

impl NodeFunAndNode {
    pub fn new() -> Self {
        Self {
            id: None,
            nid: None,
            fid: None,
        }
    }
}