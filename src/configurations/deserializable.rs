use super::parse_error::ParseError;
use std::collections::HashMap;
use std::str::FromStr;

/// Returns the parse value of a given property name
///
/// ### Errors
///  * `ErrorCantParseValue`: It will appear when the value to parse isn't in the correct format  
///  * `ErrorReadableError`: It will appear when there isn't a value with a given property name
pub(super) fn deserialize<V: FromStr>(
    name: &str,
    settings_dictionary: &HashMap<String, String>,
) -> Result<V, ParseError> {
    if let Some(value) = settings_dictionary.get(name) {
        match value.parse::<V>() {
            Ok(parse_value) => Ok(parse_value),
            _ => Err(ParseError::ErrorCantParseValue),
        }
    } else {
        Err(ParseError::ErrorReadableError)
    }
}
