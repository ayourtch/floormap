// #[macro_use] extern crate diesel;

use chrono;
use chrono::NaiveDateTime;

use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel::*;
use serde;
use std::fmt;

// use serde::ser::{Serialize, Serializer, SerializeSeq, SerializeMap};
use serde::{de, de::Error, Deserialize, Deserializer, Serialize, Serializer};

use std::str::FromStr;
use uuid;

// use diesel::sql_types::{Unsigned, Smallint};
use std::io::Write;

#[derive(AsExpression, FromSqlRow, PartialEq, Debug, Clone)]
#[sql_type = "Text"]
pub struct FlexUuid {
    Uuid: uuid::Uuid,
}

impl ToSql<Text, Sqlite> for FlexUuid {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
        let t = format!("{}", self.Uuid);
        <String as ToSql<Text, Sqlite>>::to_sql(&t, out)
    }
}

impl FromSql<Text, Sqlite> for FlexUuid {
    fn from_sql(input: Option<&<Sqlite as Backend>::RawValue>) -> deserialize::Result<Self> {
        let val_s = <String as FromSql<Text, Sqlite>>::from_sql(input)?;
        let val = uuid::Uuid::from_str(&val_s)?;
        let ret = FlexUuid { Uuid: val };
        Ok(ret)
    }
}

impl fmt::Display for FlexUuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.Uuid)
    }
}

impl Default for FlexUuid {
    fn default() -> Self {
        /* return the "now" value of naivedatetime */
        use chrono::*;
        let ndt = Local::now().naive_local();
        FlexUuid {
            Uuid: uuid::Uuid::new_v4(),
        }
    }
}

impl Serialize for FlexUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}", self.Uuid).serialize(serializer)
    }
}

struct FlexUuidVisitor;

impl<'de> de::Visitor<'de> for FlexUuidVisitor {
    type Value = FlexUuid;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string represents chrono::NaiveDateTime")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match uuid::Uuid::from_str(s) {
            Ok(t) => Ok(FlexUuid { Uuid: t }),
            Err(_) => Err(de::Error::invalid_value(de::Unexpected::Str(s), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for FlexUuid {
    fn deserialize<D>(deserializer: D) -> Result<FlexUuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlexUuidVisitor)
    }
}
