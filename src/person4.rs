use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Person {
    name: NameTuple,
    age: AgeTuple,
    is_active: IsActiveTuple,
    mails: Vec<MailAddressTuple>,
    company: Option<CompanyTuple>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct NameTuple(String);

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct AgeTuple(u8);

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct IsActiveTuple(bool);

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct MailAddressTuple(String);

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct CompanyTuple(String);

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
            name: NameTuple("Alice".to_string()),
            age: AgeTuple(42),
            is_active: IsActiveTuple(true),
            mails: vec![
                MailAddressTuple("alice@example.com".to_string()),
                MailAddressTuple("wonderland@example.com".to_string()),
            ],
            company: Some(CompanyTuple("ABC technologies".to_string())),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_json2() {
        let actual = json_to_person(JSON2).unwrap();
        let expected = Person {
            name: NameTuple("Bob".to_string()),
            age: AgeTuple(43),
            is_active: IsActiveTuple(false),
            mails: vec![MailAddressTuple("bob@example.com".to_string())],
            company: None,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_json3() {
        let actual = json_to_person(JSON3).unwrap();
        let expected = Person {
            name: NameTuple("Carol".to_string()),
            age: AgeTuple(44),
            is_active: IsActiveTuple(true),
            mails: vec![],
            company: None,
        };

        assert_eq!(actual, expected);
    }
}
