use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Person {
    name: String,
    age: u8,
    is_active: bool,
    mails: Vec<String>,
    company: Option<String>,
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
            name: "Alice".to_string(),
            age: 42,
            is_active: true,
            mails: vec![
                "alice@example.com".to_string(),
                "wonderland@example.com".to_string(),
            ],
            company: Some("ABC technologies".to_string()),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_json2() {
        let actual = json_to_person(JSON2).unwrap();
        let expected = Person {
            name: "Bob".to_string(),
            age: 43,
            is_active: false,
            mails: vec!["bob@example.com".to_string()],
            company: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_json3() {
        let actual = json_to_person(JSON3).unwrap();
        let expected = Person {
            name: "Carol".to_string(),
            age: 44,
            is_active: true,
            mails: vec![],
            company: None,
        };

        assert_eq!(actual, expected);
    }
}
