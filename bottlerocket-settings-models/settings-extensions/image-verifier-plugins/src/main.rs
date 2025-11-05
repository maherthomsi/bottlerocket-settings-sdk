use bottlerocket_settings_sdk::{BottlerocketSetting, NullMigratorExtensionBuilder};
use settings_extension_image_verifier_plugins::ImageVerifierPluginsSettingsV1;
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match NullMigratorExtensionBuilder::with_name("image-verifier-plugins")
        .with_models(vec![
            BottlerocketSetting::<ImageVerifierPluginsSettingsV1>::model(),
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
