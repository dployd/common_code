use prettytable::Cell;
use serde::{Deserialize, Serialize};

use crate::node::Node;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRow {
    pub id: String,
    pub name: String,
    pub mac_address: String,
    pub tftp_prefix: String,
    pub serial_number: String,
    pub status: Option<bool>,
    pub hostname: Option<String>,
    pub ipv4_address: Option<String>,
    pub usable: bool,
}

impl NodeRow {
    #[must_use]
    pub fn new(
        node: Node,
        status: Option<bool>,
        hostname: Option<String>,
        ipv4_address: Option<String>,
        usable: bool,
    ) -> Self {
        NodeRow {
            id: node.id,
            name: node.name,
            mac_address: node.mac_address,
            tftp_prefix: node.tftp_prefix,
            serial_number: node.serial_number,
            status,
            hostname,
            ipv4_address,
            usable,
        }
    }

    #[must_use]
    pub fn get_cells(&self) -> Vec<Cell> {
        let empty = String::from("\u{2014}");
        let hostname = self.hostname.as_ref().unwrap_or(&empty);
        let ipv4_address = self.ipv4_address.as_ref().unwrap_or(&empty);
        let vec = vec![
            self.id.as_str(),
            self.name.as_str(),
            self.mac_address.as_str(),
            self.tftp_prefix.as_str(),
            self.serial_number.as_str(),
            match self.status {
                Some(is_up) => {
                    if is_up {
                        "up"
                    } else {
                        "down"
                    }
                }
                None => "\u{2014}",
            },
            hostname.as_str(),
            ipv4_address.as_str(),
            if self.usable { "yes" } else { "no" },
        ];
        vec.into_iter().map(Cell::new).collect()
    }
}
