use crate::architecture::Architecture;
use crate::utils::get_random_name;
use chrono::{NaiveDateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use yaml_rust::yaml::Hash;
use yaml_rust::Yaml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    pub id: Option<i64>,
    pub name: String,
    pub image: String,
    pub hostname: String,
    pub replicas: i64,
    pub deployment: Option<i64>,
    pub ipv4_address: Option<String>,
    pub preferred_node: Option<String>,
    pub start: NaiveDateTime,
    pub end: Option<NaiveDateTime>,
    pub node: Option<String>,
    pub architecture: Option<Architecture>,
}

impl Service {
    #[must_use]
    pub fn new(name: &str, image: &str, hostname: &str) -> Self {
        Service {
            id: None,
            name: String::from(name),
            image: String::from(image),
            hostname: String::from(hostname),
            replicas: 1,
            deployment: None,
            ipv4_address: None,
            preferred_node: None,
            start: Utc::now().naive_local(),
            end: None,
            node: None,
            architecture: None,
        }
    }

    #[must_use]
    pub fn from_yaml(name: &str, hash: &Hash) -> Self {
        let image = String::from(
            hash.get(&Yaml::from_str("image"))
                .unwrap_or(&Yaml::String(String::new()))
                .as_str()
                .unwrap_or_default(),
        );
        let hostname = match hash.get(&Yaml::from_str("hostname")) {
            None => get_random_name(),
            Some(value) => String::from(value.as_str().unwrap_or_default()),
        };
        let replicas = match hash.get(&Yaml::from_str("replicas")) {
            Some(r) => r.as_i64().unwrap_or(1),
            None => 1,
        };
        let node = hash
            .get(&Yaml::from_str("node"))
            .map(|n| n.as_str().unwrap_or_default().to_string());
        let ipv4_address = hash
            .get(&Yaml::from_str("ipv4-address"))
            .map(|ip| ip.as_str().unwrap_or_default().to_string());
        Service {
            id: None,
            name: String::from(name),
            image,
            hostname,
            replicas,
            deployment: None,
            ipv4_address,
            preferred_node: node,
            start: Utc::now().naive_local(),
            end: None,
            node: None,
            architecture: None,
        }
    }

    #[must_use]
    pub fn from_row(row: &Row) -> Self {
        let arch: String = row.get(9).unwrap_or_else(|_| String::from("unknown"));
        Service {
            id: row.get(0).unwrap_or(Some(0)),
            name: row.get(1).unwrap_or_default(),
            image: row.get(2).unwrap_or_default(),
            deployment: row.get(3).unwrap_or(None),
            node: row.get(4).unwrap_or(None),
            start: row.get(5).unwrap_or_else(|_| Utc::now().naive_utc()),
            end: row.get(6).unwrap_or(None),
            ipv4_address: row.get(7).unwrap_or(None),
            hostname: row.get(8).unwrap_or_default(),
            replicas: 1,
            preferred_node: None,
            architecture: Some(Architecture::parse(&arch).unwrap_or(Architecture::ARM64)),
        }
    }

    #[must_use]
    pub fn group_services(services: Vec<Service>) -> Vec<Service> {
        let mut groups: HashMap<String, Vec<Service>> = HashMap::new();
        for service in services {
            let key = format!("{}-{}", service.image, service.deployment.unwrap_or(1));
            let mut value = match groups.get(key.as_str()) {
                Some(v) => v.clone(),
                None => Vec::new(),
            };
            value.push(service);
            groups.insert(key, value);
        }
        let mut vec = Vec::new();
        for (_, value) in groups {
            if let Some(first) = value.first() {
                let mut clone = first.clone();
                clone.replicas = value.len().try_into().unwrap_or(1);
                vec.push(clone);
            }
        }
        vec
    }
}
