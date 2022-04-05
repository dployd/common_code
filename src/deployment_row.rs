use chrono::NaiveDateTime;
use prettytable::Cell;
use serde::{Deserialize, Serialize};

use crate::deployment::Deployment;
use crate::utils::get_cell_content_of_date;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeploymentRow {
    pub id: Option<i64>,
    pub name: String,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub owner: String,
    pub services: Option<usize>,
}

impl DeploymentRow {
    #[must_use]
    pub fn new(deployment: Deployment, services: Option<usize>) -> Self {
        DeploymentRow {
            id: deployment.id,
            name: deployment.name,
            start: Some(deployment.start),
            end: deployment.end,
            owner: deployment.owner,
            services,
        }
    }

    #[must_use]
    pub fn get_cells(&self) -> Vec<Cell> {
        let number = match self.services {
            Some(number) => number.to_string(),
            None => String::from("\u{2014}"),
        };
        vec![
            self.id.unwrap_or_default().to_string().as_str(),
            self.name.as_str(),
            get_cell_content_of_date(&self.start).as_str(),
            get_cell_content_of_date(&self.end).as_str(),
            self.owner.as_str(),
            number.as_str(),
        ]
        .into_iter()
        .map(Cell::new)
        .collect()
    }
}
