pub mod tririga;
pub mod transport;
pub mod gui;
pub mod content;
pub mod dto;

use xsd_types::types as xs;
use crate::tririga::dto::QueryResponseHelper;
use chrono::{DateTime, FixedOffset};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

pub const TRIRIGA_JSON_FILENAME: &str = "tririga.json";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TririgaEnvironment {
    pub name: Option<String>,
    pub web_host: Option<String>,
    pub web_username: Option<String>,
    pub web_password: Option<String>,
    pub db_url: Option<String>,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub alternate_schema_name: Option<String>,
}

impl Default for TririgaEnvironment {
    fn default() -> Self {
        TririgaEnvironment {
            name: Some(String::from("Local")),
            web_host: Some(String::from("http://localhost:9080")),
            web_username: Some(String::from("system")),
            web_password: Some(String::from("admin")),
            db_url: None,
            db_username: None,
            db_password: None,
            alternate_schema_name: None
        }
    }
}




pub trait ResponseHelperExt {
    fn get_field_value(&self, field: &str) -> Option<&String>;
    fn get_date_field_value(&self, field: &str) -> Option<DateTime<FixedOffset>>;
}

impl ResponseHelperExt for QueryResponseHelper {

    fn get_field_value(&self, field: &str) -> Option<&String> {
        if let Some(columns) = &self.query_response_columns {
            let columns = &columns.query_response_column;
            for column in columns {
                if column.name == Some(field.to_string()) {
                    return column.value.as_ref();
                }
            }
        }

        None
    }

    fn get_date_field_value(&self, field: &str) -> Option<DateTime<FixedOffset>> {
        // Expected format = "Thu Jun 04 15:21:56 +09:30 2020"
        if let Some(value) = self.get_field_value(field) {

            let parts: Vec<&str>  = value.split(' ').collect();
            println!("{:?}", parts);
            let zone = parts[4];

            println!("{:?}", zone);
            let tz: Tz = zone.parse().unwrap();

            println!("{:?}", tz);

             return DateTime::parse_from_str(
                   "Thu Jun 04 15:21:56 +09:30 2020",
                   "%a %b %d %H:%M:%S %z %Y").ok()
        }

        None
    }

}