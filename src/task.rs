use chrono::NaiveDateTime;
use rusqlite::Row;
use serde::{Deserialize, Serialize};

use crate::deployment::Deployment;
use crate::service::Service;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum Type {
    NoOp = 0,
    PurgeLocalStorage = 1,
    DeleteLocalStorage = 2,
    StopIfTrue = 3,
    GetResults = 4,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub deployment: Option<Deployment>,
    pub service: Option<Service>,
    pub task_type: Type,
    pub parameters: String,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub during_deployment: bool,
}

impl Task {
    #[must_use]
    pub fn new(
        deployment: Option<Deployment>,
        service: Option<Service>,
        task_type: Type,
        parameters: String,
        during_deployment: bool,
    ) -> Self {
        Task {
            deployment,
            service,
            task_type,
            parameters,
            during_deployment,
            start: None,
            end: None,
        }
    }

    #[must_use]
    pub fn from_row(deployment: Deployment, service: Option<Service>, row: &Row) -> Option<Self> {
        if row.column_count() >= 7 {
            return Some(Task {
                deployment: Some(deployment),
                service,
                task_type: from_db_to_type(row.get(3).unwrap_or(usize::MIN)),
                parameters: row.get(4).unwrap_or_else(|_| String::from("")),
                during_deployment: row.get(5).unwrap_or(false),
                start: row.get(6).unwrap_or(None),
                end: row.get(7).unwrap_or(None),
            });
        }
        None
    }
}

fn from_db_to_type(id: usize) -> Type {
    match id {
        1 => Type::PurgeLocalStorage,
        2 => Type::DeleteLocalStorage,
        3 => Type::StopIfTrue,
        4 => Type::GetResults,
        _ => Type::NoOp,
    }
}
