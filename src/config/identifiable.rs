use std::fmt;
use serde::de::*;

use super::ident::IdentLike;
use super::regex::Regex;

pub trait Identifiable {
    fn name(&self) -> Option<&String>;
    fn pattern(&self) -> Option<&Regex>;
}

macro_rules! define_identifiable {
    ( $name: ident ) => {
        impl ::config::identifiable::Identifiable for $name {
            fn name(&self) -> Option<&String> {
                self.name.as_ref()
            }
            fn pattern(&self) -> Option<&Regex> {
                self.pattern.as_ref()
            }
        }
    }
}

impl <T: Identifiable> IdentLike for T {
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

pub fn deserialize_identifiables<'de, D, T: Identifiable + Deserialize<'de>>(de: D) -> Result<Vec<T>, D::Error>
where D: Deserializer<'de> {
    use std::marker::PhantomData;
    use serde::de::{Error, Visitor};

    struct SeqVisitor<T>(PhantomData<T>);

    impl<'de, T: Identifiable + Deserialize<'de>> Visitor<'de> for SeqVisitor<T> {
        type Value = Vec<T>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("array")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Vec<T>, A::Error> {
            let mut v = Vec::new();
            while let Some(elem) = seq.next_element::<T>()? {
                match (elem.name().is_some(), elem.pattern().is_some()) {
                    (false, false) => return Err(A::Error::missing_field("name` or `pattern")),
                    (true, true) => return Err(A::Error::custom("Both 'name' and 'pattern' given")),
                    _ => ()
                }
                v.push(elem);
            }
            Ok(v)
        }
    }

    de.deserialize_seq(SeqVisitor(PhantomData))
}
