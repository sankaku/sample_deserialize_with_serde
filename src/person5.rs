use serde::de::Deserializer;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Person {
    #[serde(deserialize_with = "deserialize_name")]
    name: Name,
    #[serde(deserialize_with = "deserialize_age")]
    age: Age,
    #[serde(deserialize_with = "deserialize_is_active")]
    is_active: IsActive,
    #[serde(deserialize_with = "deserialize_vec_mail_address")]
    mails: Vec<MailAddress>,
    #[serde(deserialize_with = "deserialize_option_company")]
    #[serde(default = "default_option_company")]
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

fn deserialize_name<'de, D>(deserializer: D) -> Result<Name, D::Error>
where
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    Ok(Name { value: v })
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

fn deserialize_mail_address<'de, D>(deserializer: D) -> Result<MailAddress, D::Error>
where
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    Ok(MailAddress { value: v })
}

fn deserialize_vec_mail_address<'de, D>(deserializer: D) -> Result<Vec<MailAddress>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_mail_address")] MailAddress);

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

fn deserialize_company<'de, D>(deserializer: D) -> Result<Company, D::Error>
where
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    Ok(Company { value: v })
}

fn deserialize_option_company<'de, D>(deserializer: D) -> Result<Option<Company>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(#[serde(deserialize_with = "deserialize_company")] Company);

    let v = Option::deserialize(deserializer)?;
    Ok(v.map(|Wrapper(a)| a))
}
fn default_option_company() -> Option<Company> {
    None
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
