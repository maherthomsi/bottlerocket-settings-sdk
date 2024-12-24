# `bottlerocket-settings-models` Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- See [unreleased changes here]

[unreleased changes here]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/compare/bottlerocket-settings-models-v0.7.0...HEAD

## [0.7.0] - 2024-12-24

## Model Changes

### Added

- Add kubernetes device ownership settings ([#69])

### Changed

- Align kubernetes cluster name validation with EKS ([#64]) Thanks @cartermckinnon

[#64]:https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/64
[#69]:https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/69


## [0.6.0] - 2024-10-02

## Model Changes

### Added

- Added nvidia time-slicing to kubernetes device plugins settings extension ([#62])

[#62]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/62

[0.6.0]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/compare/bottlerocket-settings-models-v0.6.0...bottlerocket-settings-models-v0.5.0

## [0.5.0] - 2024-09-10

## Model Changes

### Added

- Added kubernetes device plugins settings extension ([#60])

### Changed

- Drop `nvidia-device-plugin` cargo feature ([#60])

[#60]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/60

[0.5.0]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/compare/bottlerocket-settings-models-v0.5.0...bottlerocket-settings-models-v0.4.0

## [0.4.0] - 2024-09-04

## Model Changes

### Added

- Added the bootstrap-commands settings extension and related shared models ([#46])

### Changed

- Changed `bottlerocket-modeled-types::BootstrapContainerMode` to `bottlerocket-modeled-types::BootstrapMode` ([#46])

[#46]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/46

[0.4.0]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/compare/bottlerocket-settings-models-v0.4.0...bottlerocket-settings-models-v0.3.0

## [0.3.0] - 2024-08-14

## Model Changes

### Added

- Added the nvidia-container-runtime settings extension ([#43])
- Added optional nvidia device-plugins settings to kubernetes model ([#43])

### Changed

- Skipped serializing credential provider fields if they are None ([#51])
- Moved kubernetes models to a kubernetes settings extension ([#53])
- Updated dependencies ([#50], [#47])

[#43]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/43
[#47]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/47
[#50]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/50
[#51]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/51
[#53]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/53

[0.3.0]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/compare/bottlerocket-settings-models-v0.3.0...bottlerocket-settings-models-v0.2.0

## [0.2.0] - 2024-07-29

### Changed

- Added `hostname_override_source` to kubernetes settings model ([#42])

[#42]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/pull/42

[0.2.0]: https://github.com/bottlerocket-os/bottlerocket-settings-sdk/compare/bottlerocket-settings-models-v0.2.0...bottlerocket-settings-models-v0.1.0

