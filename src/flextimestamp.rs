// #[macro_use] extern crate diesel;

use chrono;
use chrono::NaiveDateTime;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgConnection};
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

use std::str::FromStr;

#[derive(AsExpression, FromSqlRow, PartialEq, Debug, Clone)]
#[sql_type = "Timestamp"]
pub struct FlexTimestamp {
    Ndt: NaiveDateTime,
}

impl FlexTimestamp {
    pub fn now() -> Self {
        Default::default()
    }
    pub fn from_timestamp(x: i64) -> Self {
        let t = NaiveDateTime::from_timestamp(x, 0);
        FlexTimestamp { Ndt: t }
    }
    pub fn timestamp(&self) -> i64 {
        self.Ndt.timestamp()
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

impl ToSql<Timestamp, Pg> for FlexTimestamp {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        let t = self.Ndt;
        <NaiveDateTime as ToSql<Timestamp, Pg>>::to_sql(&t, out)
    }
}

impl FromSql<Timestamp, Pg> for FlexTimestamp {
    fn from_sql(input: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        let ndt = <NaiveDateTime as FromSql<Timestamp, Pg>>::from_sql(input)?;
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
            Err(_) => match NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
                Ok(t) => Ok(FlexTimestamp { Ndt: t }),
                Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
            },
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

impl FromStr for FlexTimestamp {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.%f")?;
        Ok(FlexTimestamp { Ndt: t })
    }
}
