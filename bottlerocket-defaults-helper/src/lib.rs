/// This helper library generates a unified TOML file representing the default settings for the
/// system.  The contents of that file are used by storewolf to populate the defaults into the data
/// store on a new system.
///
/// The goal of generating the defaults file like this is to allow to break up and share groups of
/// default settings, without having to ship those files in the OS image. We read any number of
/// files from a defaults.d directory in the variant's model directory and merge later entries into
/// earlier entries, so later files take precedence.
mod merge_toml;

use merge_toml::merge_values;
use snafu::ResultExt;
use std::env::var as getenv;
use std::fs;
use std::path::PathBuf;
use toml::{map::Map, Value};
use walkdir::WalkDir;

/// Merge the variant's default settings files into a single TOML value.  The result is serialized
/// to a file in OUT_DIR for storewolf to read.
pub fn generate_defaults_toml() -> Result<()> {
    let mut defaults_dir = PathBuf::new();
    defaults_dir.push(getenv("CARGO_MANIFEST_DIR").context(error::CargoEnvSnafu {
        var: "CARGO_MANIFEST_DIR".to_string(),
    })?);
    defaults_dir.push("defaults.d");

    // Reflect that we need to rerun if any of the default settings files have changed.
    println!("cargo::rerun-if-changed={}", defaults_dir.display());

    // Find TOML config files specified by the variant.
    let walker = WalkDir::new(&defaults_dir)
        .follow_links(true) // we expect users to link to shared files
        .min_depth(1) // only read files in defaults.d, not doing inheritance yet
        .max_depth(1)
        .sort_by(|a, b| a.file_name().cmp(b.file_name())) // allow ordering by prefix
        .into_iter()
        .filter_entry(|e| e.file_name().to_string_lossy().ends_with(".toml")); // looking for TOML config

    // Merge the files into a single TOML value, in order.
    let mut defaults = Value::Table(Map::new());
    for entry in walker {
        let entry = entry.context(error::ListFilesSnafu { dir: &defaults_dir })?;

        let data = fs::read_to_string(entry.path()).context(error::FileSnafu {
            op: "read",
            path: entry.path(),
        })?;
        let value =
            toml::from_str(&data).context(error::TomlDeserializeSnafu { path: entry.path() })?;
        merge_values(&mut defaults, &value).context(error::TomlMergeSnafu)?;
    }

    // Serialize to disk for storewolf to read.
    let data = toml::to_string(&defaults).context(error::TomlSerializeSnafu)?;
    let mut output_file = PathBuf::new();
    output_file.push(getenv("OUT_DIR").context(error::CargoEnvSnafu {
        var: "OUT_DIR".to_string(),
    })?);
    output_file.push(format!(
        "{}.toml",
        getenv("CARGO_PKG_NAME").context(error::CargoEnvSnafu {
            var: "CARGO_PKG_NAME".to_string()
        })?
    ));
    fs::write(&output_file, data).context(error::FileSnafu {
        op: "write",
        path: output_file,
    })?;

    Ok(())
}

mod error {
    use crate::merge_toml;
    use snafu::Snafu;
    use std::path::PathBuf;

    #[derive(Debug, Snafu)]
    #[snafu(visibility(pub(super)))]
    pub enum Error {
        #[snafu(display("Environment variable {} not set: are you not using cargo?", var))]
        CargoEnv {
            var: String,
            source: std::env::VarError,
        },

        #[snafu(display("Failed to {} {}: {}", op, path.display(), source))]
        File {
            op: String,
            path: PathBuf,
            source: std::io::Error,
        },

        #[snafu(display("Failed to list files in {}: {}", dir.display(), source))]
        ListFiles {
            dir: PathBuf,
            source: walkdir::Error,
        },

        #[snafu(display("{} is not valid TOML: {}", path.display(), source))]
        TomlDeserialize {
            path: PathBuf,
            source: toml::de::Error,
        },

        #[snafu(display("Failed to merge TOML: {}", source))]
        TomlMerge { source: merge_toml::Error },

        #[snafu(display("Failed to serialize default settings: {}", source))]
        TomlSerialize { source: toml::ser::Error },
    }
}

pub type Result<T> = std::result::Result<T, error::Error>;
