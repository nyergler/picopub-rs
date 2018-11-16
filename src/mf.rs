extern crate serde;
extern crate serde_json;

use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serde::de::{self, Deserialize, Deserializer, SeqAccess, Visitor};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "properties")]
pub enum MFEntry {

    #[serde(rename = "h-entry")]
    Entry(EntryProps),
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "type", deserialize_with = "string_or_array")]
    entry_type: String,

    properties: EntryProps,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EntryProps {
    #[serde(default)]
    title: String,
    // summary String;
    #[serde(deserialize_with = "string_or_array")]
    content: String,
    // Author String;
    // URL String;
    #[serde(rename = "category")]
    categories: Vec<String>,
}

fn string_or_array<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or single element array")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            match FromStr::from_str(value) {
                Err(_) => Err(E::custom("foo")),
                Ok(s) => Ok(s),
            }
            // Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<T, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let first = seq.next_element()?.ok_or_else(||
                // Cannot take the maximum of an empty seq.
                de::Error::custom("no values in seq when looking for fist"))?;

            Ok(first)
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[cfg(test)]
mod tests {
    use crate::mf;

    #[test]
    fn test_json_deserialize() {
        let input_json = r#"
        {
            "type": ["h-entry"],
            "properties": {
                "content": ["hello world"],
                "category": ["foo","bar"]
            }
        }"#;

        let entry: mf::MFEntry = serde_json::from_str(input_json).unwrap();
        if let mf::MFEntry::Entry(e) = entry {
        assert_eq!(e.content, "hello world");
        assert_eq!(e.categories, ["foo", "bar"]);
            
        }
    }

    #[test]
    fn test_urlencode_deserialize() {}
}
