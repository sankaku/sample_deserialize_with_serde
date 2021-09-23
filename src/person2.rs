use serde::de::{self, Visitor};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Person {
    name: Name,
    age: Age,
    is_active: IsActive,
    mails: Vec<MailAddress>,
    company: Option<Company>,
}

#[derive(Debug, PartialEq, Eq)]
struct Name {
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Age {
    value: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct IsActive {
    value: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct MailAddress {
    value: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Company {
    value: String,
}

struct NameVisitor;
impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("String for Name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Name {
            value: v.to_string(),
        })
    }
}
impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Name, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NameVisitor)
    }
}

struct AgeVisitor;
impl<'de> Visitor<'de> for AgeVisitor {
    type Value = Age;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Integer for Age")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Age { value: v as u8 })
    }
}
impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(deserializer: D) -> Result<Age, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u64(AgeVisitor)
    }
}

struct IsActiveVisitor;
impl<'de> Visitor<'de> for IsActiveVisitor {
    type Value = IsActive;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Bool for IsActive")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(IsActive { value: v })
    }
}
impl<'de> Deserialize<'de> for IsActive {
    fn deserialize<D>(deserializer: D) -> Result<IsActive, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bool(IsActiveVisitor)
    }
}

struct MailAddressVisitor;
impl<'de> Visitor<'de> for MailAddressVisitor {
    type Value = MailAddress;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("String for MailAddress")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(MailAddress {
            value: v.to_string(),
        })
    }
}
impl<'de> Deserialize<'de> for MailAddress {
    fn deserialize<D>(deserializer: D) -> Result<MailAddress, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(MailAddressVisitor)
    }
}

struct CompanyVisitor;
impl<'de> Visitor<'de> for CompanyVisitor {
    type Value = Company;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("String for Company")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Company {
            value: v.to_string(),
        })
    }
}
impl<'de> Deserialize<'de> for Company {
    fn deserialize<D>(deserializer: D) -> Result<Company, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(CompanyVisitor)
    }
}

pub fn json_to_person(json: &str) -> Result<Person, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jsons::*;

    #[test]
    fn test_json1() {
        let actual = json_to_person(JSON1).unwrap();
        let expected = Person {
            name: Name {
                value: "Alice".to_string(),
            },
            age: Age { value: 42 },
            is_active: IsActive { value: true },
            mails: vec![
                MailAddress {
                    value: "alice@example.com".to_string(),
                },
                MailAddress {
                    value: "wonderland@example.com".to_string(),
                },
            ],
            company: Some(Company {
                value: "ABC technologies".to_string(),
            }),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_json2() {
        let actual = json_to_person(JSON2).unwrap();
        let expected = Person {
            name: Name {
                value: "Bob".to_string(),
            },
            age: Age { value: 43 },
            is_active: IsActive { value: false },
            mails: vec![MailAddress {
                value: "bob@example.com".to_string(),
            }],
            company: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_json3() {
        let actual = json_to_person(JSON3).unwrap();
        let expected = Person {
            name: Name {
                value: "Carol".to_string(),
            },
            age: Age { value: 44 },
            is_active: IsActive { value: true },
            mails: vec![],
            company: None,
        };

        assert_eq!(actual, expected);
    }
}
