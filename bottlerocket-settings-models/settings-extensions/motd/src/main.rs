use bottlerocket_settings_sdk::{BottlerocketSetting, LinearMigratorExtensionBuilder};
use settings_extension_motd::MotdV1;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match LinearMigratorExtensionBuilder::with_name("motd")
        .with_models(vec![BottlerocketSetting::<MotdV1>::model()])
        .build()
    {
        Ok(extension) => extension.run(),
        Err(e) => {
            println!("{e}");
            ExitCode::FAILURE
        }
    }
}
