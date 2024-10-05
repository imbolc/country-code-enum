use serde::{de::Error, Deserialize, Deserializer, Serializer};

use super::CountryCode;

impl<'de> Deserialize<'de> for CountryCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.as_str().parse().map_err(Error::custom)
    }
}

impl serde::Serialize for CountryCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(serde_json::to_string(&CountryCode::AR).unwrap(), "\"AR\"");
    }

    #[test]
    fn deserialize_uppercase() {
        assert_eq!(
            serde_json::from_str::<CountryCode>("\"AR\"").unwrap(),
            CountryCode::AR
        );
    }

    #[test]
    fn deserialize_lowercase() {
        assert_eq!(
            serde_json::from_str::<CountryCode>("\"ar\"").unwrap(),
            CountryCode::AR
        );
    }

    #[test]
    fn deserialize_error() {
        assert!(serde_json::from_str::<CountryCode>("\"BAD\"").is_err(),);
    }
}
