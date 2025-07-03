//! Settings related to bootstrap commands.
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{ApiclientCommand, BootstrapMode, Identifier};
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeMap, convert::Infallible};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BootstrapCommandsSettingsV1 {
    pub bootstrap_commands: BTreeMap<Identifier, BootstrapCommand>,
}

// Custom serializer/deserializer added to maintain backwards
// compatibility with models created prior to settings extensions.
impl Serialize for BootstrapCommandsSettingsV1 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.bootstrap_commands.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BootstrapCommandsSettingsV1 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bootstrap_commands = BTreeMap::deserialize(deserializer)?;
        Ok(Self { bootstrap_commands })
    }
}

#[model(impl_default = true)]
struct BootstrapCommand {
    commands: Vec<ApiclientCommand>,
    mode: BootstrapMode,
    essential: bool,
}

impl SettingsModel for BootstrapCommandsSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that parses as BootstrapCommandsSettingsV1.
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
        // Validate anything that parses as BootstrapCommandsSettingsV1.
        Ok(())
    }
}

type Result<T> = std::result::Result<T, Infallible>;

#[cfg(test)]
mod test_bootstrap_command {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_bootstrap_command_settings() {
        let generated = BootstrapCommandsSettingsV1::generate(None, None).unwrap();

        assert_eq!(
            generated,
            GenerateResult::Complete(BootstrapCommandsSettingsV1 {
                bootstrap_commands: BTreeMap::new(),
            })
        )
    }

    #[test]
    fn test_serde_bootstrap_command() {
        let test_json = json!({
            "mybootstrap": {
                "commands": [ ["apiclient", "motd=hello"], ],
                "mode": "once",
                "essential": true,
            }
        });

        let bootstrap_commands: BootstrapCommandsSettingsV1 =
            serde_json::from_value(test_json.clone()).unwrap();

        let mut expected_bootstrap_commands: BTreeMap<Identifier, BootstrapCommand> =
            BTreeMap::new();
        expected_bootstrap_commands.insert(
            Identifier::try_from("mybootstrap").unwrap(),
            BootstrapCommand {
                commands: Some(vec![ApiclientCommand::try_from(vec![
                    "apiclient".to_string(),
                    "motd=hello".to_string(),
                ])
                .unwrap()]),
                mode: Some(BootstrapMode::try_from("once").unwrap()),
                essential: Some(true),
            },
        );

        assert_eq!(
            bootstrap_commands,
            BootstrapCommandsSettingsV1 {
                bootstrap_commands: expected_bootstrap_commands
            }
        );

        let serialized_json: serde_json::Value = serde_json::to_string(&bootstrap_commands)
            .map(|s| serde_json::from_str(&s).unwrap())
            .unwrap();

        assert_eq!(serialized_json, test_json);
    }

    #[test]
    fn test_serde_invalid_bootstrap_command() {
        let test_err_json = json!({
            "mybootstrap1": {
                "commands": [ ["/usr/bin/touch", "helloworld"], ],
                "mode": "once",
                "essential": true,
            }
        });

        let bootstrap_commands_err: std::result::Result<
            BootstrapCommandsSettingsV1,
            serde_json::Error,
        > = serde_json::from_value(test_err_json.clone());

        // This has invalid command. It should fail.
        assert!(bootstrap_commands_err.is_err());
    }
}
