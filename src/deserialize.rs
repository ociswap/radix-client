use serde_path_to_error;

/// Special deserialize function that uses serde_path_to_error to provide more detailed error messages
/// in case of a deserialization error.
pub fn from_str<T>(
    text: &str,
) -> Result<T, serde_path_to_error::Error<serde_json::Error>>
where
    T: serde::de::DeserializeOwned,
{
    let deserializer = &mut serde_json::Deserializer::from_str(&text);
    serde_path_to_error::deserialize(deserializer)
}
