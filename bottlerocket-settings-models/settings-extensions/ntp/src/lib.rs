//! The ntp settings can be used to specify time servers with which to synchronize the instance's
//! clock.
use bottlerocket_model_derive::model;
use bottlerocket_modeled_types::{Identifier, Url};
use bottlerocket_settings_sdk::{GenerateResult, LinearlyMigrateable, NoMigration, SettingsModel};
use std::collections::HashMap;
use std::convert::Infallible;

// ============ V1 (existing, now migrates forward to V2) ============

#[model(impl_default = true)]
pub struct NtpSettingsV1 {
    time_servers: Vec<Url>,
    options: Vec<String>,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for NtpSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
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
        Ok(())
    }
}

impl LinearlyMigrateable for NtpSettingsV1 {
    type ForwardMigrationTarget = NtpSettingsV2;
    type BackwardMigrationTarget = NoMigration;

    fn migrate_forward(&self) -> Result<Self::ForwardMigrationTarget> {
        let mut time_servers = HashMap::new();

        if let Some(servers) = &self.time_servers {
            for (i, server) in servers.iter().enumerate() {
                let key = Identifier::try_from(format!("time-server-{}", i))
                    .unwrap_or_else(|_| Identifier::try_from("time-server-0").unwrap());
                time_servers.insert(
                    key,
                    NtpTimeServer {
                        address: Some(server.clone()),
                        directive: None,
                        prefer: None,
                        options: self.options.clone(),
                        minpoll: None,
                        maxpoll: None,
                    },
                );
            }
        }

        Ok(NtpSettingsV2 {
            time_servers: Some(time_servers),
        })
    }

    fn migrate_backward(&self) -> Result<Self::BackwardMigrationTarget> {
        NoMigration::no_defined_migration()
    }
}

// ============ V2 (new, per-server configuration) ============

/// Individual time server configuration
#[model(impl_default = true)]
pub struct NtpTimeServer {
    /// Server address (IP or hostname)
    address: Url,
    /// "server" or "pool" - defaults to "server" if unset
    directive: String,
    /// Mark as preferred NTP source
    prefer: bool,
    /// Per-server options like ["iburst"]
    options: Vec<String>,
    /// Minimum polling interval (log2 seconds, e.g. 4 = 16s)
    minpoll: i32,
    /// Maximum polling interval (log2 seconds, e.g. 4 = 16s)
    maxpoll: i32,
}

/// V2 NTP settings with per-server configuration (HashMap pattern)
#[model(impl_default = true)]
pub struct NtpSettingsV2 {
    time_servers: HashMap<Identifier, NtpTimeServer>,
}

impl SettingsModel for NtpSettingsV2 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v2"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
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
        Ok(())
    }
}

impl LinearlyMigrateable for NtpSettingsV2 {
    type ForwardMigrationTarget = NoMigration;
    type BackwardMigrationTarget = NtpSettingsV1;

    fn migrate_forward(&self) -> Result<Self::ForwardMigrationTarget> {
        NoMigration::no_defined_migration()
    }

    fn migrate_backward(&self) -> Result<Self::BackwardMigrationTarget> {
        let mut time_servers = Vec::new();
        let mut options = Vec::new();

        if let Some(servers) = &self.time_servers {
            for (_key, server) in servers {
                if let Some(addr) = &server.address {
                    time_servers.push(addr.clone());
                }
                // Carry over options from first server that has them
                if options.is_empty() {
                    if let Some(opts) = &server.options {
                        options = opts.clone();
                    }
                }
            }
        }

        Ok(NtpSettingsV1 {
            time_servers: if time_servers.is_empty() {
                None
            } else {
                Some(time_servers)
            },
            options: if options.is_empty() {
                None
            } else {
                Some(options)
            },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_ntp_settings_v1() {
        assert_eq!(
            NtpSettingsV1::generate(None, None),
            Ok(GenerateResult::Complete(NtpSettingsV1 {
                time_servers: None,
                options: None,
            }))
        );
    }

    #[test]
    fn test_generate_ntp_settings_v2() {
        assert_eq!(
            NtpSettingsV2::generate(None, None),
            Ok(GenerateResult::Complete(NtpSettingsV2 {
                time_servers: None,
            }))
        );
    }

    #[test]
    fn test_serde_ntp_v2() {
        let test_json = r#"{"time-servers":{"link-local":{"address":"169.254.169.123","directive":"server","prefer":true,"options":["iburst"],"minpoll":4,"maxpoll":4}}}"#;
        let ntp: NtpSettingsV2 = serde_json::from_str(test_json).unwrap();
        let servers = ntp.time_servers.unwrap();
        let link_local = servers.get(&Identifier::try_from("link-local").unwrap()).unwrap();
        assert_eq!(link_local.prefer, Some(true));
        assert_eq!(link_local.minpoll, Some(4));
        assert_eq!(link_local.maxpoll, Some(4));
    }

    #[test]
    fn test_v1_to_v2_migration() {
        let v1 = NtpSettingsV1 {
            time_servers: Some(vec![
                Url::try_from("169.254.169.123").unwrap(),
                Url::try_from("time.aws.com").unwrap(),
            ]),
            options: Some(vec!["iburst".to_string()]),
        };
        let v2 = v1.migrate_forward().unwrap();
        let servers = v2.time_servers.unwrap();
        assert_eq!(servers.len(), 2);
    }

    #[test]
    fn test_v2_to_v1_migration() {
        let mut servers = HashMap::new();
        servers.insert(
            Identifier::try_from("link-local").unwrap(),
            NtpTimeServer {
                address: Some(Url::try_from("169.254.169.123").unwrap()),
                directive: Some("server".to_string()),
                prefer: Some(true),
                options: Some(vec!["iburst".to_string()]),
                minpoll: Some(4),
                maxpoll: Some(4),
            },
        );
        let v2 = NtpSettingsV2 { time_servers: Some(servers) };
        let v1 = v2.migrate_backward().unwrap();
        assert_eq!(v1.time_servers.unwrap().len(), 1);
        assert_eq!(v1.options.unwrap(), vec!["iburst"]);
    }

    #[test]
    fn test_serde_ntp() {
        let test_json = r#"{"time-servers":["https://example.net","http://www.example.com"]}"#;

        let ntp: NtpSettingsV1 = serde_json::from_str(test_json).unwrap();
        assert_eq!(
            ntp.time_servers.clone().unwrap(),
            vec!(
                Url::try_from("https://example.net").unwrap(),
                Url::try_from("http://www.example.com").unwrap(),
            )
        );

        let results = serde_json::to_string(&ntp).unwrap();
        assert_eq!(results, test_json);
    }

    #[test]
    fn test_options_ntp() {
        let test_json = r#"{"time-servers":["https://example.net","http://www.example.com"],"options":["minpoll","1","maxpoll","2"]}"#;

        let ntp: NtpSettingsV1 = serde_json::from_str(test_json).unwrap();
        assert_eq!(
            ntp.options.clone().unwrap(),
            vec!("minpoll", "1", "maxpoll", "2",)
        );

        let results = serde_json::to_string(&ntp).unwrap();
        assert_eq!(results, test_json);
    }
}
