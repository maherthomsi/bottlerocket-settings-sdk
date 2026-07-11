use bottlerocket_settings_sdk::{BottlerocketSetting, LinearMigratorExtensionBuilder};
use settings_extension_ntp::{NtpSettingsV1, NtpSettingsV2};
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();
    match LinearMigratorExtensionBuilder::with_name("ntp")
        .with_models(vec![
            BottlerocketSetting::<NtpSettingsV1>::model(),
            BottlerocketSetting::<NtpSettingsV2>::model(),
        ])
        .build()
    {
        Ok(extension) => extension.run(),
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
    }
}
