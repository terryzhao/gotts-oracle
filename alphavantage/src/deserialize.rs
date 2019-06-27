use chrono::prelude::*;
use chrono_tz::Tz;
use failure::{err_msg, Error};
use serde::de::{self, Deserialize, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

pub(crate) const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub(crate) const DATE_FORMAT: &str = "%Y-%m-%d";

pub(crate) fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub(crate) fn parse_date(value: &str, time_zone: Tz) -> Result<DateTime<Tz>, Error> {
    if value.contains(':') {
        let datetime = NaiveDateTime::parse_from_str(value, DATETIME_FORMAT)?;
        time_zone
            .from_local_datetime(&datetime)
            .single()
            .ok_or_else(|| err_msg("unable to parse datetime"))
    } else {
        let datetime = NaiveDate::parse_from_str(value, DATE_FORMAT).map(|d| d.and_hms(0, 0, 0))?;
        time_zone
            .from_local_datetime(&datetime)
            .single()
            .ok_or_else(|| err_msg("unable to parse date"))
    }
}
