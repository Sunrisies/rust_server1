use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 船坞任务实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockyardNew {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "is_out")]
    pub is_out: Option<bool>,
    #[serde(rename = "is_stop")]
    pub is_stop: Option<bool>,
    pub start_time: Option<Vec<String>>,
    pub air_route: Option<Vec<String>>,
    pub boat_id: Option<String>,
    pub port: Option<i32>,
    pub state: Option<String>,
    pub target_destination: Option<Vec<Vec<String>>>,
    #[serde(rename = "errorCause")]
    pub error_cause: Option<String>,
    #[serde(rename = "boatName")]
    pub boat_name: Option<String>,
    #[serde(rename = "addDate")]
    pub add_date: Option<DateTime<Utc>>,
    #[serde(rename = "updateDate")]
    pub update_date: Option<DateTime<Utc>>,
    #[serde(rename = "taskAttribute")]
    pub task_attribute: Option<String>,
}

impl DockyardNew {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            is_out: Some(false),
            is_stop: Some(false),
            start_time: Some(Vec::new()),
            air_route: Some(Vec::new()),
            boat_id: None,
            port: None,
            state: Some("1".to_string()),
            target_destination: Some(Vec::new()),
            error_cause: None,
            boat_name: None,
            add_date: Some(Utc::now()),
            update_date: None,
            task_attribute: None,
        }
    }
}