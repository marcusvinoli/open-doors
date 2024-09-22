use std::fmt;

use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Visitor};

use super::definitions;

#[derive(Clone, Debug)]
pub struct SemVer {
    major: usize,
    minor: usize,
    patch: usize,
}

impl SemVer {
    pub fn from<S: AsRef<str>>(ver: S) -> Self {
        let vers = ver.as_ref().to_string();
        let mut vers = vers.split(".").into_iter();
        SemVer {
            major: vers.next().unwrap_or("1").parse::<usize>().unwrap_or(1),
            minor: vers.next().unwrap_or("0").parse::<usize>().unwrap_or(0),
            patch: vers.next().unwrap_or("0").parse::<usize>().unwrap_or(0),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Serialize for SemVer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SemVerVisitor)
    }
}

impl Default for SemVer {
    fn default() -> Self {
        Self::from(definitions::OD_DEFAULT_BASELINE_SEM_VER)
    }
}

struct SemVerVisitor;

impl<'de> Visitor<'de> for SemVerVisitor {
    type Value = SemVer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A version string in the format 'major.minor.patch'")
    }

    fn visit_str<E>(self, value: &str) -> Result<SemVer, E>
    where
        E: de::Error,
    {
        let parts: Vec<&str> = value.split('.').collect();
        if parts.len() != 3 {
            return Err(de::Error::invalid_length(parts.len(), &self));
        }

        let major = parts[0].parse::<usize>().map_err(de::Error::custom)?;
        let minor = parts[1].parse::<usize>().map_err(de::Error::custom)?;
        let patch = parts[2].parse::<usize>().map_err(de::Error::custom)?;

        Ok(SemVer { major, minor, patch })
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Baseline {
    pub version: SemVer,
    pub description: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub commit: Option<String>,
}
