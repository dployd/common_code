use std::collections::HashMap;

use config::Value;
use serde::{Deserialize, Serialize};

use crate::architecture::Architecture;
use crate::logsource::LogSource;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub tftp_prefix: String,
    pub mac_address: String,
    pub serial_number: String,
    pub ipv4_address: String,
    pub log_inputs: Vec<LogSource>,
    pub architecture: Architecture,
    pub pxe: bool,
}

impl Node {
    #[must_use]
    pub fn from_config(
        id: String,
        hash: &HashMap<String, Value>,
        log_inputs: Vec<LogSource>,
    ) -> Self {
        Node {
            id,
            name: hash
                .get("name")
                .unwrap_or(&Value::from("node"))
                .clone()
                .into_str()
                .unwrap_or_default(),
            tftp_prefix: hash
                .get("tftp-prefix")
                .unwrap_or(&Value::from("00-00-00-00-00-00"))
                .clone()
                .into_str()
                .unwrap_or_default(),
            mac_address: hash
                .get("mac-address")
                .unwrap_or(&Value::from("00-00-00-00-00-00"))
                .clone()
                .into_str()
                .unwrap_or_default(),
            ipv4_address: hash
                .get("ipv4-address")
                .unwrap_or(&Value::from("0.0.0.0"))
                .clone()
                .into_str()
                .unwrap_or_default(),
            serial_number: hash
                .get("serial-number")
                .unwrap_or(&Value::from("00-00-00-00-00-00"))
                .clone()
                .into_str()
                .unwrap_or_default(),
            log_inputs,
            architecture: Architecture::parse(
                &hash
                    .get("architecture")
                    .unwrap_or(&Value::from("ARM64"))
                    .clone()
                    .into_str()
                    .unwrap_or_default(),
            )
            .unwrap_or(Architecture::ARM64),
            pxe: if hash.contains_key("pxe") {
                hash.get("pxe")
                    .unwrap_or(&Value::from("false"))
                    .clone()
                    .into_bool()
                    .unwrap_or(false)
            } else {
                false
            },
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
            && self.name.eq(&other.name)
            && self.tftp_prefix.eq(&other.tftp_prefix)
            && self.mac_address.eq(&other.mac_address)
            && self.serial_number.eq(&other.serial_number)
    }
}
