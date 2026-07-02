//! Settings for controlling what is excluded from TPM PCR measurements.
//!
//! The `settings.measurement` namespace lets customers configure which settings
//! are excluded from PCR 8 measurement by rottweiler.
//!
//! Each entry in `excluded-settings` names a settings node. Specifying a node
//! excludes it and the entire tree beneath it — for example, `"host-containers"`
//! excludes all settings under `settings.host-containers`. Wildcards are not
//! supported; specify the node directly.
use bottlerocket_model_derive::model;
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use snafu::{ensure, Snafu};

/// Patterns that must never be allowed as exclusions because they would
/// undermine the integrity of the measurement system.
const DISALLOWED_PATTERNS: &[&str] = &["settings", ""];

#[derive(Debug, Snafu, PartialEq)]
pub enum MeasurementValidationError {
    #[snafu(display("Disallowed measurement exclusion pattern: {:?}", pattern))]
    DisallowedPattern { pattern: String },

    #[snafu(display("Self-referencing measurement exclusion not allowed: {:?}", pattern))]
    SelfReference { pattern: String },

    #[snafu(display("Wildcard not allowed in measurement exclusion: {:?} — specify the top-level setting node to exclude its entire tree", pattern))]
    ContainsWildcard { pattern: String },
}

/// Normalize an exclusion value by stripping a leading "settings." prefix if present.
/// Users may provide either "host-containers" or "settings.host-containers".
fn normalize_exclusion(value: &str) -> &str {
    value.strip_prefix("settings.").unwrap_or(value)
}

/// Check whether a normalized exclusion is a self-reference to the measurement namespace.
fn is_self_reference(normalized: &str) -> bool {
    normalized == "measurement" || normalized.starts_with("measurement.")
}

/// Validate that a single exclusion pattern is allowed.
fn validate_exclusion(value: &str) -> std::result::Result<(), MeasurementValidationError> {
    let trimmed = value.trim();
    let normalized = normalize_exclusion(trimmed);
    ensure!(
        !DISALLOWED_PATTERNS.contains(&normalized),
        DisallowedPatternSnafu {
            pattern: trimmed.to_string(),
        }
    );
    ensure!(
        !is_self_reference(normalized),
        SelfReferenceSnafu {
            pattern: trimmed.to_string(),
        }
    );
    ensure!(
        !trimmed.contains('*'),
        ContainsWildcardSnafu {
            pattern: trimmed.to_string(),
        }
    );
    Ok(())
}

#[model(impl_default = true)]
pub struct MeasurementSettingsV1 {
    excluded_settings: Vec<String>,
}

type Result<T> = std::result::Result<T, MeasurementValidationError>;

impl SettingsModel for MeasurementSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = MeasurementValidationError;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Allow anything that parses as MeasurementSettingsV1.
        Ok(())
    }

    fn generate(
        existing_partial: Option<Self::PartialKind>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialKind, Self>> {
        Ok(GenerateResult::Complete(
            existing_partial.unwrap_or_default(),
        ))
    }

    fn validate(value: Self, _validated_settings: Option<serde_json::Value>) -> Result<()> {
        if let Some(ref excluded) = value.excluded_settings {
            for entry in excluded {
                validate_exclusion(entry)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_measurement() {
        let generated = MeasurementSettingsV1::generate(None, None).unwrap();
        assert_eq!(
            generated,
            GenerateResult::Complete(MeasurementSettingsV1 {
                excluded_settings: None,
            })
        )
    }

    #[test]
    fn test_serde_measurement() {
        let test_json = r#"{"excluded-settings":["host-containers","bootstrap-containers"]}"#;

        let measurement: MeasurementSettingsV1 = serde_json::from_str(test_json).unwrap();
        assert_eq!(
            measurement,
            MeasurementSettingsV1 {
                excluded_settings: Some(vec![
                    String::from("host-containers"),
                    String::from("bootstrap-containers"),
                ]),
            }
        );

        let results = serde_json::to_string(&measurement).unwrap();
        assert_eq!(results, test_json);
    }

    #[test]
    fn test_validate_allows_valid_patterns() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![
                String::from("host-containers"),
                String::from("bootstrap-containers"),
                String::from("network.hostname"),
            ]),
        };
        assert!(MeasurementSettingsV1::validate(value, None).is_ok());
    }

    #[test]
    fn test_validate_allows_settings_prefix() {
        // Users can write "settings.host-containers" — validation accepts it
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![
                String::from("settings.host-containers"),
                String::from("bootstrap-containers"),
            ]),
        };
        assert!(MeasurementSettingsV1::validate(value, None).is_ok());
    }

    #[test]
    fn test_validate_rejects_bare_settings() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![
                String::from("host-containers"),
                String::from("settings"),
            ]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::DisallowedPattern {
                pattern: String::from("settings"),
            })
        );
    }

    #[test]
    fn test_validate_rejects_empty_string() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![String::from("")]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::DisallowedPattern {
                pattern: String::from(""),
            })
        );
    }

    #[test]
    fn test_validate_rejects_wildcard() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![String::from("*")]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::ContainsWildcard {
                pattern: String::from("*"),
            })
        );
    }

    #[test]
    fn test_validate_rejects_glob_pattern() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![String::from("host-containers.*")]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::ContainsWildcard {
                pattern: String::from("host-containers.*"),
            })
        );
    }

    #[test]
    fn test_validate_rejects_self_reference() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![String::from("measurement")]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::SelfReference {
                pattern: String::from("measurement"),
            })
        );
    }

    #[test]
    fn test_validate_rejects_self_reference_subpath() {
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![String::from("measurement.excluded-settings")]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::SelfReference {
                pattern: String::from("measurement.excluded-settings"),
            })
        );
    }

    #[test]
    fn test_validate_rejects_self_reference_with_settings_prefix() {
        // "settings.measurement" normalizes to "measurement" which is self-referencing
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![String::from("settings.measurement")]),
        };
        let result = MeasurementSettingsV1::validate(value, None);
        assert_eq!(
            result,
            Err(MeasurementValidationError::SelfReference {
                pattern: String::from("settings.measurement"),
            })
        );
    }

    #[test]
    fn test_validate_allows_none() {
        let value = MeasurementSettingsV1 {
            excluded_settings: None,
        };
        assert!(MeasurementSettingsV1::validate(value, None).is_ok());
    }

    #[test]
    fn test_validate_does_not_reject_measurement_prefix_words() {
        // "measurements" and "measurement-foo" are not the measurement namespace
        let value = MeasurementSettingsV1 {
            excluded_settings: Some(vec![
                String::from("measurements"),
                String::from("measurement-foo"),
            ]),
        };
        assert!(MeasurementSettingsV1::validate(value, None).is_ok());
    }
}
