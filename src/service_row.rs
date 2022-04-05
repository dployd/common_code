use chrono::NaiveDateTime;
use prettytable::Cell;
use serde::{Deserialize, Serialize};

use crate::service::Service;
use crate::utils::{get_cell_content_of_date, get_cell_content_of_option};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceRow {
    pub id: i64,
    pub name: String,
    pub image: String,
    pub node: String,
    pub deployment: String,
    pub hostname: String,
    pub ipv4_address: Option<String>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub replicas: i64,
}

impl ServiceRow {
    #[must_use]
    pub fn new(service: &Service, node: String, deployment: String) -> Self {
        ServiceRow {
            id: service.id.unwrap_or(0),
            name: service.name.clone(),
            image: service.image.clone(),
            node,
            deployment,
            hostname: service.hostname.clone(),
            ipv4_address: service.ipv4_address.clone(),
            start: Some(service.start),
            end: service.end,
            replicas: service.replicas,
        }
    }

    #[must_use]
    pub fn get_cells(&self) -> Vec<Cell> {
        let id = self.id.to_string();
        let replicas = self.replicas.to_string();
        let ipv4_address = get_cell_content_of_option(&self.ipv4_address);
        let start = get_cell_content_of_date(&self.start);
        let vec = vec![
            id.as_str(),
            &self.name,
            &self.image,
            &self.node,
            &self.deployment,
            &self.hostname,
            ipv4_address.as_str(),
            start.as_str(),
            replicas.as_str(),
        ];
        vec.iter().map(|c| Cell::new(c)).collect()
    }
}
