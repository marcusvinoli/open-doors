use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;

#[derive(Clone, Default, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted_string = format!("{} <{}>", self.name, self.email);
        serializer.serialize_str(&formatted_string)
    }
}

struct UserVisitor;

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format 'name <email>'")
    }

    fn visit_str<E>(self, value: &str) -> Result<User, E>
    where
        E: de::Error,
    {
        let parts: Vec<&str> = value.splitn(2, " <").collect();
        if parts.len() == 2 && parts[1].ends_with('>') {
            let name = parts[0].to_string();
            let email = parts[1][..parts[1].len() - 1].to_string();
            Ok(User { name, email })
        } else {
            Err(E::custom(format!("invalid format for User: {}", value)))
        }
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UserVisitor)
    }
}
