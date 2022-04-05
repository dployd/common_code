use chrono::{NaiveDateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use yaml_rust::Yaml;

use crate::service::Service;
use crate::task::Task;
use crate::task::Type::StopIfTrue;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deployment {
    pub id: Option<i64>,
    pub name: String,
    pub services: Vec<Service>,
    pub owner: String,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
    pub tasks: Vec<Task>,
}

impl Deployment {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Deployment {
            id: None,
            name: String::from(name),
            services: Vec::new(),
            owner: String::from("aresch"), //TODO implement owner
            start: Utc::now().naive_local(),
            end: None,
            tasks: Vec::new(),
        }
    }

    #[must_use]
    pub fn from_yaml(name: &str, yaml: &Yaml) -> Self {
        let mut services: Vec<Service> = Vec::new();
        if let Some(hash) = yaml["services"].as_hash() {
            for (n, data) in hash.iter() {
                if let Some(h) = data.as_hash() {
                    services.push(Service::from_yaml(n.as_str().unwrap_or_default(), h));
                }
            }
        }
        let mut tasks = Vec::new();
        if let Some(tasks_yaml) = yaml["stop"].as_hash() {
            for (task, data) in tasks_yaml.iter() {
                if task.as_str().unwrap_or_default().eq("log") {
                    if let Some(vec) = data.clone().into_vec() {
                        for task in vec {
                            let msg_yaml = task["message"].clone();
                            let message = msg_yaml.as_str().unwrap_or_default();
                            let occurrence =
                                task["occurrence"].clone().as_i64().unwrap_or_default();
                            let deployment: Option<Deployment> = None;
                            let service: Option<Service> = None;
                            let task = Task::new(
                                deployment,
                                service,
                                StopIfTrue,
                                serde_json::to_string(&(message, occurrence)).unwrap_or_default(),
                                true,
                            );
                            tasks.push(task);
                        }
                    }
                }
            }
        }
        Deployment {
            id: None,
            name: String::from(name),
            services,
            owner: String::from("aresch"),
            start: Utc::now().naive_local(),
            end: None,
            tasks,
        }
    }

    #[must_use]
    pub fn from_row(row: &Row) -> Self {
        Deployment {
            id: row.get(0).unwrap_or_default(),
            name: row.get(1).unwrap_or_default(),
            services: Vec::new(),
            owner: row.get(2).unwrap_or_default(),
            start: row.get(3).unwrap_or_else(|_| Utc::now().naive_utc()),
            end: row.get(4).unwrap_or(None),
            tasks: Vec::new(),
        }
    }

    #[must_use]
    pub fn get_services(&self) -> Vec<Service> {
        self.services.clone()
    }
}
