// #[macro_use] extern crate diesel;

use chrono;
use chrono::NaiveDateTime;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel::*;
use serde;
use std::fmt;

// use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
use serde::{de, de::Error, Deserialize, Deserializer, Serialize, Serializer};

// use diesel::sql_types::{Unsigned, Smallint};
use diesel::sql_types::Timestamp;
use std::io::Write;

#[derive(AsExpression, FromSqlRow, PartialEq, Debug, Clone)]
#[sql_type = "Timestamp"]
pub struct FlexTimestamp {
    Ndt: NaiveDateTime,
}

impl FlexTimestamp {
    pub fn now() -> Self {
        Default::default()
    }
}

impl ToSql<Timestamp, Sqlite> for FlexTimestamp {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let t = self.Ndt;
        <NaiveDateTime as ToSql<Timestamp, Sqlite>>::to_sql(&t, out)
    }
}

impl FromSql<Timestamp, Sqlite> for FlexTimestamp {
    fn from_sql(input: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let ndt = <NaiveDateTime as FromSql<Timestamp, Sqlite>>::from_sql(input)?;
        let ret = FlexTimestamp { Ndt: ndt };
        Ok(ret)
    }
}

impl Default for FlexTimestamp {
    fn default() -> Self {
        /* return the "now" value of naivedatetime */
        use chrono::*;
        let ndt = Local::now().naive_local();
        FlexTimestamp { Ndt: ndt }
    }
}

impl Serialize for FlexTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.Ndt.serialize(serializer)
    }
}

struct FlexTimestampVisitor;

impl<'de> de::Visitor<'de> for FlexTimestampVisitor {
    type Value = FlexTimestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string represents chrono::NaiveDateTime")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.%f") {
            Ok(t) => Ok(FlexTimestamp { Ndt: t }),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for FlexTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<FlexTimestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlexTimestampVisitor)
    }
}
