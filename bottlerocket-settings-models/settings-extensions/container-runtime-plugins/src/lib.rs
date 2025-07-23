//! Settings related to Container Runtime Plugins
mod de;

use crate::de::deserialize_chunk_size;
use bottlerocket_model_derive::model;
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[model(impl_default = true)]
pub struct ContainerRuntimePluginsSettingsV1 {
    soci_snapshotter: SociSnapshotterSettings,
}

#[model(impl_default = true)]
pub struct SociSnapshotterSettings {
    pull_mode: SociSnapshotterPullMode,
    parallel_pull_unpack: SociSnapshotterParallelPullUnpack,
}

#[model(impl_default = true)]
pub struct SociSnapshotterParallelPullUnpack {
    max_concurrent_downloads: i64,
    max_concurrent_downloads_per_image: i64,
    concurrent_download_chunk_size: ChunkSize,
    max_concurrent_unpacks: i64,
    max_concurrent_unpacks_per_image: i64,
    discard_unpacked_layers: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
enum ChunkSize {
    #[default]
    Unlimited,
    #[serde(deserialize_with = "deserialize_chunk_size", untagged)]
    Bytes(i64),
}

/// SociSnapshotterPullMode is used to select which pull mode is enabled.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SociSnapshotterPullMode {
    #[default]
    ParallelPullUnpack,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for ContainerRuntimePluginsSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that can be parsed as ContainerRuntimePluginsSettingsV1.
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
        // ContainerRuntimePluginsSettingsV1 is validated during deserialization.
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_container_runtime_plugins_settings() {
        assert_eq!(
            ContainerRuntimePluginsSettingsV1::generate(None, None),
            Ok(GenerateResult::Complete(
                ContainerRuntimePluginsSettingsV1 {
                    soci_snapshotter: None,
                }
            ))
        )
    }

    #[test]
    fn test_serde_container_runtime_plugins() {
        let test_json = json!({
            "soci-snapshotter": {
                "pull-mode": "parallel-pull-unpack",
                "parallel-pull-unpack": {
                    "max-concurrent-downloads": 11,
                    "concurrent-download-chunk-size": "64mb",
                    "max-concurrent-unpacks-per-image": 4,
                    "discard-unpacked-layers": false
                }
            }
        });

        let test_json_str = test_json.to_string();

        let container_runtime_plugins_settings: ContainerRuntimePluginsSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        assert_eq!(
            container_runtime_plugins_settings,
            ContainerRuntimePluginsSettingsV1 {
                soci_snapshotter: Some(SociSnapshotterSettings {
                    pull_mode: Some(SociSnapshotterPullMode::ParallelPullUnpack),
                    parallel_pull_unpack: Some(SociSnapshotterParallelPullUnpack {
                        max_concurrent_downloads: Some(11),
                        max_concurrent_downloads_per_image: None,
                        concurrent_download_chunk_size: Some(ChunkSize::Bytes(64000000)), // 64mb in bytes
                        max_concurrent_unpacks: None,
                        max_concurrent_unpacks_per_image: Some(4),
                        discard_unpacked_layers: Some(false),
                    }),
                }),
            }
        );
        let test_json_unlimited = json!({
            "soci-snapshotter": {
                "pull-mode": "parallel-pull-unpack",
                "parallel-pull-unpack": {
                    "max-concurrent-downloads": 11,
                    "concurrent-download-chunk-size": "unlimited",
                    "max-concurrent-unpacks-per-image": 4,
                    "discard-unpacked-layers": false
                }
            }
        });

        let test_json_unlimited_str = test_json_unlimited.to_string();

        let container_runtime_plugins_settings: ContainerRuntimePluginsSettingsV1 =
            serde_json::from_str(&test_json_unlimited_str).unwrap();

        assert_eq!(
            container_runtime_plugins_settings,
            ContainerRuntimePluginsSettingsV1 {
                soci_snapshotter: Some(SociSnapshotterSettings {
                    pull_mode: Some(SociSnapshotterPullMode::ParallelPullUnpack),
                    parallel_pull_unpack: Some(SociSnapshotterParallelPullUnpack {
                        max_concurrent_downloads: Some(11),
                        max_concurrent_downloads_per_image: None,
                        concurrent_download_chunk_size: Some(ChunkSize::Unlimited),
                        max_concurrent_unpacks: None,
                        max_concurrent_unpacks_per_image: Some(4),
                        discard_unpacked_layers: Some(false),
                    }),
                }),
            }
        );
    }

    #[test]
    fn test_serde_container_runtime_plugins_soci_fields_optional() {
        let test_json = json!({
            "soci-snapshotter": {
                "pull-mode": "parallel-pull-unpack",
                "parallel-pull-unpack": {
                }
            }
        });

        let test_json_str = test_json.to_string();

        let container_runtime_plugins_settings: ContainerRuntimePluginsSettingsV1 =
            serde_json::from_str(&test_json_str).unwrap();

        assert_eq!(
            container_runtime_plugins_settings,
            ContainerRuntimePluginsSettingsV1 {
                soci_snapshotter: Some(SociSnapshotterSettings {
                    pull_mode: Some(SociSnapshotterPullMode::ParallelPullUnpack),
                    parallel_pull_unpack: Some(SociSnapshotterParallelPullUnpack {
                        max_concurrent_downloads: None,
                        max_concurrent_downloads_per_image: None,
                        concurrent_download_chunk_size: None,
                        max_concurrent_unpacks: None,
                        max_concurrent_unpacks_per_image: None,
                        discard_unpacked_layers: None,
                    }),
                }),
            }
        );
    }

    #[test]
    fn test_concurrent_download_chunk_size_validation() {
        // Test valid size strings
        let valid_sizes = vec![
            "8b", "16kb", "64mb", "1gb", "1.5gb", "100", "42.7MB", ".5gb",
        ];

        for size in valid_sizes {
            let test_json = json!({
                "soci-snapshotter": {
                    "parallel-pull-unpack": {
                        "max-concurrent-downloads": 0,
                        "max-concurrent-downloads-per-image": 0,
                        "concurrent-download-chunk-size": size,
                        "max-concurrent-unpacks": 0,
                        "max-concurrent-unpacks-per-image": 0,
                        "discard-unpacked-layers": false
                    }
                }
            });

            let result: std::result::Result<ContainerRuntimePluginsSettingsV1, _> =
                serde_json::from_str(&test_json.to_string());
            assert!(result.is_ok(), "Failed to parse valid size string: {size}");
        }

        // Test that the field can be none.
        let test_json = json!({
        "soci-snapshotter": {
            "parallel-pull-unpack": {
                "max-concurrent-downloads": 0,
                "max-concurrent-downloads-per-image": 0,
                "max-concurrent-unpacks": 0,
                "max-concurrent-unpacks-per-image": 0,
                "discard-unpacked-layers": false
            }
        }});

        let result: std::result::Result<ContainerRuntimePluginsSettingsV1, _> =
            serde_json::from_str(&test_json.to_string());
        assert!(
            result.is_ok(),
            "Failed to deserialize None concurrent-download-chunk-size"
        );

        // Test valid integer values
        let valid_integers = vec![1024, 2048, 0];

        for size in valid_integers {
            let test_json = json!({
                "soci-snapshotter": {
                    "parallel-pull-unpack": {
                        "max-concurrent-downloads": 0,
                        "max-concurrent-downloads-per-image": 0,
                        "concurrent-download-chunk-size": size,
                        "max-concurrent-unpacks": 0,
                        "max-concurrent-unpacks-per-image": 0,
                        "discard-unpacked-layers": false
                    }
                }
            });

            let result: std::result::Result<ContainerRuntimePluginsSettingsV1, _> =
                serde_json::from_str(&test_json.to_string());
            assert!(result.is_ok(), "Failed to parse valid integer: {size}");
        }

        // Test invalid size strings
        let invalid_sizes = vec!["", "abc", "8.5.2mb", "1000 KB", "2.5 GB", "10 mb", "-1"];

        for size in invalid_sizes {
            let test_json = json!({
                "soci-snapshotter": {
                    "parallel-pull-unpack": {
                        "max-concurrent-downloads": 0,
                        "max-concurrent-downloads-per-image": 0,
                        "concurrent-download-chunk-size": size,
                        "max-concurrent-unpacks": 0,
                        "max-concurrent-unpacks-per-image": 0,
                        "discard-unpacked-layers": false
                    }
                }
            });

            let result: std::result::Result<ContainerRuntimePluginsSettingsV1, _> =
                serde_json::from_str(&test_json.to_string());
            assert!(
                result.is_err(),
                "Should have failed to parse invalid size string: {size}"
            );
        }

        // Test invalid integer values
        let invalid_integers = vec![-2, -100];

        for size in invalid_integers {
            let test_json = json!({
                "soci-snapshotter": {
                    "parallel-pull-unpack": {
                        "max-concurrent-downloads": 0,
                        "max-concurrent-downloads-per-image": 0,
                        "concurrent-download-chunk-size": size,
                        "max-concurrent-unpacks": 0,
                        "max-concurrent-unpacks-per-image": 0,
                        "discard-unpacked-layers": false
                    }
                }
            });

            let result: std::result::Result<ContainerRuntimePluginsSettingsV1, _> =
                serde_json::from_str(&test_json.to_string());
            assert!(
                result.is_err(),
                "Should have failed to parse invalid integer: {size}"
            );
        }
    }
}
