pub mod tririga;
pub mod transport;
pub mod gui;
pub mod content;
pub mod dto;

use xsd_types::types as xs;
use crate::tririga::dto::QueryResponseHelper;
use chrono::{DateTime, FixedOffset};
use chrono_tz::Tz;

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