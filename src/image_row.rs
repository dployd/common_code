use crate::configuration::Configuration;
use prettytable::Cell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageRow {
    pub filename: String,
    pub configuration: Option<Configuration>,
}

impl ImageRow {
    #[must_use]
    pub fn new(filename: String, configuration: Option<Configuration>) -> Self {
        ImageRow {
            filename,
            configuration,
        }
    }

    #[must_use]
    pub fn get_cells(&self) -> Vec<Cell> {
        let mut cells = vec![self.filename.as_str()];
        match &self.configuration {
            Some(config) => {
                cells.push(config.architecture.get_name());
                cells.push(if config.on_device { "yes" } else { "no" });
            }
            None => {
                cells.push("\u{2014}");
                cells.push("\u{2014}");
            }
        }
        cells.into_iter().map(Cell::new).collect()
    }
}
