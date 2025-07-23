use byte_unit::Byte;
use serde::{self, Deserialize, Deserializer};

/// This function deserializes a size string (e.g., "64mb") or integer into an i64 value
/// representing bytes.
///
/// Valid formats:
/// - Integer values (e.g., "1024" or 1024)
/// - Size strings with units (e.g., "64mb", "1.5gb", ".5gb")
///
/// Invalid formats:
/// - Negative values
/// - Strings with invalid formats
/// - Strings with whitespace
pub fn deserialize_chunk_size<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }

    let value = StringOrInt::deserialize(deserializer)?;

    match value {
        StringOrInt::String(s) => {
            let s = s.trim();

            // Try parsing as a plain integer (bytes)
            if let Ok(bytes) = s.parse::<i64>() {
                if bytes < 0 {
                    return Err(serde::de::Error::custom(format!(
                        "Size cannot be negative: {s}"
                    )));
                }
                return Ok(bytes);
            }

            if s.contains(char::is_whitespace) {
                return Err(serde::de::Error::custom(format!(
                    "Size string cannot contain whitespace: '{s}'"
                )));
            }

            // Handle the special case of ".5gb" format (byte_unit expects "0.5gb")
            let normalized_s = if s.starts_with('.') {
                format!("0{s}")
            } else {
                s.to_string()
            };

            // Try parsing as a size string with units using byte_unit, ignoring case.
            match Byte::parse_str(&normalized_s, true) {
                Ok(byte) => {
                    // Convert to i64, ensuring we don't overflow
                    let bytes = byte.as_u64();
                    i64::try_from(bytes).map_err(|e| {
                        serde::de::Error::custom(format!("Unable to convert bytes u64 to i64: {e}"))
                    })
                }
                Err(e) => Err(serde::de::Error::custom(format!(
                    "Invalid size string format '{s}': {e}"
                ))),
            }
        }
        StringOrInt::Int(i) => {
            if i < 0 {
                return Err(serde::de::Error::custom(format!(
                    "Size cannot be negative: {i}"
                )));
            }
            Ok(i)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Serialize;
    use serde_json;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "deserialize_chunk_size")]
        size: i64,
    }

    #[test]
    fn test_deserialize_chunk_size() {
        // Test valid size strings
        let test_cases = [
            ("1b", 1),
            ("1kb", 1000),
            ("1mb", 1000000),
            ("1gb", 1000000000),
            ("1tb", 1000000000000),
            ("1KiB", 1024),
            ("1MiB", 1048576),
            ("1GiB", 1073741824),
            ("1TiB", 1099511627776),
            ("1024", 1024),
            ("64mb", 64000000),
            ("1.5gb", 1500000000),
            (".5gb", 500000000), // Special case with leading decimal
        ];

        for (input, expected) in test_cases {
            let json = format!(r#"{{"size": "{}"}}"#, input);
            let test: TestStruct = serde_json::from_str(&json).unwrap();
            assert_eq!(
                test.size, expected,
                "Failed for input: {}. Expected {} bytes but got {:?}",
                input, expected, test.size
            );
        }

        // Test valid integer values
        let int_test_cases = [(1024, 1024), (0, 0)];

        for (input, expected) in int_test_cases {
            let json = format!(r#"{{"size": {}}}"#, input);
            let test: TestStruct = serde_json::from_str(&json).unwrap();
            assert_eq!(
                test.size, expected,
                "Failed for input: {}. Expected {} bytes but got {:?}",
                input, expected, test.size
            );
        }

        // Test invalid size strings
        let invalid_inputs = [
            "",
            "abc",
            "8 tb",    // Contains whitespace
            "8.5.2mb", // Invalid format - multiple decimals
            "8 x",
            "hello world",
            "-1",      // No negative value
            "1000 KB", // Contains whitespace
            "2.5 GB",  // Contains whitespace
            "10 mb",   // Contains whitespace
        ];

        for input in invalid_inputs {
            let json = format!(r#"{{"size": "{}"}}"#, input);
            let result = serde_json::from_str::<TestStruct>(&json);
            assert!(
                result.is_err(),
                "Should have failed to parse invalid size string: {}",
                input
            );
        }

        // Test invalid integer values
        let invalid_ints = [-1, -100];

        for input in invalid_ints {
            let json = format!(r#"{{"size": {}}}"#, input);
            let result = serde_json::from_str::<TestStruct>(&json);
            assert!(
                result.is_err(),
                "Should have failed to parse invalid integer: {}",
                input
            );
        }
    }

    #[test]
    fn test_with_toml() {
        // Test string format
        let toml_str = r#"size = "64mb""#;
        let test: TestStruct = toml::from_str(toml_str).unwrap();
        assert_eq!(test.size, 64 * 1000 * 1000);

        // Test integer format
        let toml_int = r#"size = 1024"#;
        let test: TestStruct = toml::from_str(toml_int).unwrap();
        assert_eq!(test.size, 1024);

        // Test decimal format with leading decimal point
        let toml_decimal = r#"size = ".5gb""#;
        let test: TestStruct = toml::from_str(toml_decimal).unwrap();
        assert_eq!(test.size, 500000000);
    }
}
