use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Person {
    #[serde(rename = "id")]
    id: u32,
    name: String,
}

#[cfg(feature = "json")]
mod json {
    use crate::Person;
    use serde_rw::FromFile;

    #[test]
    fn from_file() {
        assert_eq!(
            Person::from_file("./tests/person.json").unwrap(),
            Person {
                id: 1337,
                name: "John Doe".to_string()
            },
        )
    }
}

#[cfg(feature = "xml")]
mod xml {
    use crate::Person;
    use serde_rw::FromFile;

    #[test]
    fn from_file() {
        assert_eq!(
            Person::from_file("./tests/person.xml").unwrap(),
            Person {
                id: 1337,
                name: "John Doe".to_string()
            },
        )
    }
}

#[cfg(feature = "yaml")]
mod yaml {
    use crate::Person;
    use serde_rw::FromFile;

    #[test]
    fn from_file() {
        assert_eq!(
            Person::from_file("./tests/person.yml").unwrap(),
            Person {
                id: 1337,
                name: "John Doe".to_string()
            },
        )
    }
}
