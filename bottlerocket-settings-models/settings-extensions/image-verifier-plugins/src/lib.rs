//! Settings related to Image Verifier Plugins

use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::ValidBase64;
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use std::convert::Infallible;

#[model(impl_default = true)]
pub struct ImageVerifierPluginsSettingsV1 {
    enabled: bool,
    notation: NotationSettings,
}

#[model(impl_default = true)]
pub struct NotationSettings {
    /// Base64 encoded trustpolicy.json
    /// https://github.com/notaryproject/specifications/blob/main/specs/trust-store-trust-policy.md#trust-store
    trustpolicy: ValidBase64,
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

    #[test]
    fn test_generate_image_verifier_plugins_settings() {
        assert_eq!(
            ImageVerifierPluginsSettingsV1::generate(None, None),
            Ok(GenerateResult::Complete(ImageVerifierPluginsSettingsV1 {
                enabled: None,
                notation: None
            }))
        )
    }

    #[test]
    fn test_serde_image_verifier_plugins() {
        let test_json = json!({
            "enabled": true,
            "notation": {
                "trustpolicy": "ewogICJ2ZXJzaW9uIjogIjEuMCIsCiAgInRydXN0UG9saWNpZXMiOiBbXQp9"
            }
        });

        let test_json_str = test_json.to_string();

        let image_verifier_plugins_settings: ImageVerifierPluginsSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        assert_eq!(image_verifier_plugins_settings.enabled, Some(true));
        assert_eq!(
            image_verifier_plugins_settings
                .notation
                .as_ref()
                .unwrap()
                .trustpolicy
                .as_ref()
                .unwrap()
                .as_ref(),
            "ewogICJ2ZXJzaW9uIjogIjEuMCIsCiAgInRydXN0UG9saWNpZXMiOiBbXQp9"
        );
    }

    #[test]
    fn test_serde_image_verifier_plugins_empty() {
        let test_json = json!({});

        let test_json_str = test_json.to_string();

        let image_verifier_plugins_settings: ImageVerifierPluginsSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        assert!(image_verifier_plugins_settings.enabled.is_none());
        assert!(image_verifier_plugins_settings.notation.is_none());
    }

    #[test]
    fn test_serde_image_verifier_plugins_enabled_only() {
        let test_json = json!({
            "enabled": false
        });

        let test_json_str = test_json.to_string();

        let image_verifier_plugins_settings: ImageVerifierPluginsSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        assert_eq!(image_verifier_plugins_settings.enabled, Some(false));
        assert!(image_verifier_plugins_settings.notation.is_none());
    }

    #[test]
    fn test_serde_image_verifier_plugins_invalid_base64() {
        let test_json = json!({
            "notation": {
                "trustpolicy": "invalid-base64!"
            }
        });

        let test_json_str = test_json.to_string();

        let result = serde_json::from_str::<ImageVerifierPluginsSettingsV1>(&test_json_str);

        assert!(result.is_err());
    }
}
