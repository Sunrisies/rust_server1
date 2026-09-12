use serde::{Deserialize, Serialize};

/// 船只权限实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePermission {
    pub id: Option<String>,
    pub name: Option<String>,
    pub code: Option<i32>,
    pub details: Option<String>,
    // 查询参数
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub keywords: Option<String>,
}

impl NodePermission {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            code: None,
            details: None,
            page: None,
            limit: None,
            keywords: None,
        }
    }
}

/// 船只权限关联实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAndPermission {
    pub id: Option<String>,
    pub nodeid: Option<String>,
    pub jurid: Option<String>,
}

impl NodeAndPermission {
    pub fn new() -> Self {
        Self {
            id: None,
            nodeid: None,
            jurid: None,
        }
    }
}