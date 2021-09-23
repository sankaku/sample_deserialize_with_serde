use serde::de::Deserializer;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::str::FromStr;
use std::string::ParseError;

#[serde_as]
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Person {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    name: Name,
    #[serde(deserialize_with = "deserialize_age")]
    age: Age,
    #[serde(deserialize_with = "deserialize_is_active")]
    is_active: IsActive,
    #[serde_as(deserialize_as = "Vec<DisplayFromStr>")]
    mails: Vec<MailAddress>,
    #[serde_as(deserialize_as = "Option<DisplayFromStr>")]
    #[serde(default = "default_option_company")]
    company: Option<Company>,
}

#[derive(Debug, PartialEq, Eq)]
struct Name {
    value: String,
}

impl FromStr for Name {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Name {
            value: s.to_string(),
        })
    }
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

fn deserialize_age<'de, D>(deserializer: D) -> Result<Age, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u8::deserialize(deserializer)?;
    Ok(Age { value: v })
}
fn deserialize_is_active<'de, D>(deserializer: D) -> Result<IsActive, D::Error>
where
    D: Deserializer<'de>,
{
    let v = bool::deserialize(deserializer)?;
    Ok(IsActive { value: v })
}
fn default_option_company() -> Option<Company> {
    None
}

impl FromStr for MailAddress {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MailAddress {
            value: s.to_string(),
        })
    }
}

impl FromStr for Company {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Company {
            value: s.to_string(),
        })
    }
}

pub fn json_to_person(json: &str) -> Result<Person, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    // Same as person2
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
