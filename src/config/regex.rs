use std::fmt;
use std::ops;
use regex;
use serde;

use library::Nullable;

//TODO: rename PatternRegex ?
#[derive(Clone, Debug)]
pub struct Regex(regex::Regex);

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, regex::Error> {
        regex::Regex::new(pattern)
            .map(Regex)
    }
}

impl ops::Deref for Regex {
    type Target = regex::Regex;
    fn deref(&self) -> &regex::Regex { &self.0 }
}

impl PartialEq for Regex {
    fn eq(&self, other: &Regex) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for Regex {}

impl<'de> serde::Deserialize<'de> for Regex {
    fn deserialize<D>(de: D) -> Result<Regex, D::Error>
    where D: serde::Deserializer<'de>
    {
        use serde::de::{Error, Visitor};

        struct RegexVisitor;

        impl<'de> Visitor<'de> for RegexVisitor {
            type Value = Regex;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                println!("in expecting");
                f.write_str("a regular expression pattern")
            }

            fn visit_str<E: Error>(self, v: &str) -> Result<Regex, E> {
                println!("in visit {:?}", v);
                regex::Regex::new(&format!("^{}$", v)).map(Regex).map_err(|err| {
                    E::custom(err.to_string())
                })
            }
        }

        de.deserialize_str(RegexVisitor)
    }
}

impl<'de> serde::Deserialize<'de> for Nullable {
    fn deserialize<D>(de: D) -> Result<Nullable, D::Error>
    where D: serde::Deserializer<'de>
    {
        use serde::de::{Error, Visitor};

        struct NullableVisitor;

        impl<'de> Visitor<'de> for NullableVisitor {
            type Value = Nullable;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                println!("in expecting");
                f.write_str("boolean")
            }

            fn visit_bool<E: Error>(self, v: bool) -> Result<Nullable, E> {
                println!("in visit {:?}", v);
                Ok(Nullable(v))
            }
        }

        de.deserialize_bool(NullableVisitor)
    }
}
