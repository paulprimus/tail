use crate::error::{OplError, OplErrorKind};
use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

const SERIALIZE_FORMAT: &'static str = "%d.%m.%Y";

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Debug)]
pub struct OplDate(Date<Utc>);

impl Deref for OplDate {
    type Target = Date<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OplDate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<Date<Utc>> for OplDate {
    fn into(self) -> Date<Utc> {
        self.0
    }
}

impl From<Date<Utc>> for OplDate {
    fn from(date: Date<Utc>) -> Self {
        Self(date)
    }
}

impl From<String> for OplDate {
    fn from(s: String) -> Self {
        //let splitted_date_str: Vec<&str> = s.split(".").collect();
        let now = Utc::now();
        let d = NaiveDate::parse_from_str(s.as_str(), SERIALIZE_FORMAT)
            .map_err(|e| OplError::new(OplErrorKind::SerdeError(e.to_string())))
            .map(|s| Date::from_utc(s, now.offset().to_owned())).unwrap()
            ;
        Self::from(d)
    }
}

impl Serialize for OplDate {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self.format(SERIALIZE_FORMAT));
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for OplDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)
            .map_err(|e| OplError::new(OplErrorKind::SerdeError(e.to_string())))
            .unwrap();
        Ok(Self::from(s))
    }
}
