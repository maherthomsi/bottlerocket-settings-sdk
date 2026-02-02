//! Settings related to Image Verifier Plugins

use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{Identifier, ValidBase64Json};
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;

/// Settings for image verifier plugins
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ImageVerifierPluginsSettingsV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(flatten)]
    pub plugins: HashMap<Identifier, VerifierPluginConfig>,
}

/// Configuration for a verifier plugin
#[model(impl_default = true)]
pub struct VerifierPluginConfig {
    #[serde(alias = "trust-policy", skip_serializing_if = "Option::is_none")]
    trustpolicy: ValidBase64Json,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for ImageVerifierPluginsSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that can be parsed as ImageVerifierPluginsSettingsV1.
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

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<()> {
        // ImageVerifierPluginsSettingsV1 is validated during deserialization.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    const NOTATION_POLICY: &str = "ewogICJ2ZXJzaW9uIjogIjEuMCIsCiAgInRydXN0UG9saWNpZXMiOiBbXQp9";

    use std::convert::TryInto;
    use test_case::test_case;

    #[test]
    fn test_generate_image_verifier_plugins_settings() {
        assert_eq!(
            ImageVerifierPluginsSettingsV1::generate(None, None),
            Ok(GenerateResult::Complete(ImageVerifierPluginsSettingsV1 {
                enabled: None,
                plugins: HashMap::new()
            }))
        )
    }

    #[test_case(json!({}), None, &[] ; "empty object")]
    #[test_case(json!({"enabled": false}), Some(false), &[] ; "enabled only")]
    #[test_case(json!({"enabled": true, "notation": {"trustpolicy": NOTATION_POLICY}}), Some(true), &[("notation", NOTATION_POLICY)] ; "single plugin")]
    #[test_case(json!({"notation": {"trustpolicy": NOTATION_POLICY}, "digestion": {"trustpolicy": "e30="}}), None, &[("notation", NOTATION_POLICY), ("digestion", "e30=")] ; "multiple plugins")]
    #[test_case(json!({"notation": {"trust-policy": NOTATION_POLICY}}), None, &[("notation", NOTATION_POLICY)] ; "trust-policy alias")]
    fn test_serde_image_verifier_plugins(
        input: serde_json::Value,
        expected_enabled: Option<bool>,
        expected_policies: &[(&str, &str)],
    ) {
        let settings: ImageVerifierPluginsSettingsV1 = serde_json::from_value(input).unwrap();
        assert_eq!(settings.enabled, expected_enabled);
        assert_eq!(settings.plugins.len(), expected_policies.len());
        for (plugin_name, expected_policy) in expected_policies {
            let key: Identifier = (*plugin_name).try_into().unwrap();
            let plugin = settings.plugins.get(&key).expect(plugin_name);
            assert_eq!(
                plugin.trustpolicy.as_ref().unwrap().as_ref(),
                *expected_policy
            );
        }
    }

    #[test_case(json!({"notation": {"trustpolicy": "invalid-base64!"}}) ; "invalid base64")]
    #[test_case(json!({"notation": {"trustpolicy": "bm90IGpzb24="}}) ; "invalid json in base64")]
    fn test_serde_invalid_trustpolicy(input: serde_json::Value) {
        assert!(serde_json::from_value::<ImageVerifierPluginsSettingsV1>(input).is_err());
    }

    #[test]
    fn test_default_serialization_is_empty() {
        let settings = ImageVerifierPluginsSettingsV1::default();
        let json = serde_json::to_value(&settings).unwrap();
        assert_eq!(
            json,
            json!({}),
            "default settings should serialize to empty object"
        );
    }

    #[test]
    fn test_plugin_with_no_trustpolicy_omits_null() {
        // A plugin registered without a trust policy should serialize
        // without a "trustpolicy": null entry.
        let input = json!({"notation": {}});
        let settings: ImageVerifierPluginsSettingsV1 = serde_json::from_value(input).unwrap();
        let output = serde_json::to_value(&settings).unwrap();
        assert_eq!(
            output,
            json!({"notation": {}}),
            "plugin with no trustpolicy should not serialize null fields"
        );
    }
}
