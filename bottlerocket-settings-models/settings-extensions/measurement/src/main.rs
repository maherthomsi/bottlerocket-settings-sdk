use bottlerocket_settings_sdk::{BottlerocketSetting, NullMigratorExtensionBuilder};
use settings_extension_measurement::MeasurementSettingsV1;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match NullMigratorExtensionBuilder::with_name("measurement")
        .with_models(vec![BottlerocketSetting::<MeasurementSettingsV1>::model()])
        .build()
    {
        Ok(extension) => extension.run(),
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
    }
}
