use std::fmt;
use regex::Regex;
use toml::Value;
use serde::de::*;

use super::regex::Regex as Regex2;
use super::error::TomlHelper;

#[derive(Clone, Debug)]
pub enum Ident {
    Name(String),
    Pattern(Regex),
}

impl PartialEq for Ident {
    fn eq(&self, other: &Ident) -> bool {
        pub use self::Ident::*;
        match (self, other) {
            (&Name(ref s1), &Name(ref s2)) => s1 == s2,
            (&Pattern(ref r1), &Pattern(ref r2)) => r1.as_str() == r2.as_str(),
            _ => false,
        }
    }
}

impl Eq for Ident {}

impl Ident {
    pub fn parse(toml: &Value, object_name: &str, what: &str) -> Option<Ident> {
        match toml.lookup("pattern").and_then(|v| v.as_str()) {
            Some(s) => Regex::new(&format!("^{}$", s))
                .map(Ident::Pattern)
                .map_err(|e| {
                    error!(
                        "Bad pattern `{}` in {} for `{}`: {}",
                        s,
                        what,
                        object_name,
                        e
                    );
                    e
                })
                .ok(),
            None => toml.lookup("name")
                .and_then(|val| val.as_str())
                .map(|s| Ident::Name(s.into())),
        }
    }

    pub fn is_match(&self, name: &str) -> bool {
        use self::Ident::*;
        match *self {
            Name(ref n) => name == n,
            Pattern(ref regex) => regex.is_match(name),
        }
    }
}



pub trait IdentLike {
    fn is_match(&self, name: &str) -> bool;
}

impl IdentLike for Ident {
    fn is_match(&self, name: &str) -> bool {
        use self::Ident::*;
        match *self {
            Name(ref n) => name == n,
            Pattern(ref regex) => regex.is_match(name),
        }
    }
}

pub trait IdentLike2 {
    fn name(&self) -> Option<&String>;
    fn pattern(&self) -> Option<&Regex2>;
}

macro_rules! ident_like2 {
    ( $name: ident ) => {
        impl IdentLike2 for $name {
            fn name(&self) -> Option<&String> {
                self.name.as_ref()
            }
            fn pattern(&self) -> Option<&Regex> {
                self.pattern.as_ref()
            }
        }
    }
}

impl <T: IdentLike2> IdentLike for T {
    fn is_match(&self, name: &str) -> bool {
        if let Some(n) = self.name() {
            name == n
        } else if let Some(regex) = self.pattern() {
            regex.is_match(name)
        } else {
            unreachable!();
        }
    }
}

pub fn deserialize_identlikes<'de, D, T: IdentLike2 + Deserialize<'de>>(de: D) -> Result<Vec<T>, D::Error>
where D: Deserializer<'de> {
    use std::marker::PhantomData;
    use serde::de::{Error, Visitor};

    struct ArrayVisitor<T>(PhantomData<T>);

    impl<T> ArrayVisitor<T> {
        pub fn new() -> Self {
            ArrayVisitor(PhantomData)
        }
    }

    impl<'de, T: IdentLike2 + Deserialize<'de>> Visitor<'de> for ArrayVisitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("array")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<T>, A::Error> {
            let mut v = Vec::new();
            while let Some(elem) = seq.next_element::<T>()? {
                match (elem.name().is_some(), elem.pattern().is_some()) {
                    (false, false) => return Err(A::Error::custom("No 'name' or 'pattern' given")),
                    (true, true) => return Err(A::Error::custom("Both 'name' and 'pattern' given")),                     _ => ()
                }
                v.push(elem);
            }
            Ok(v)
        }
    }

    de.deserialize_seq(ArrayVisitor::new())
}
