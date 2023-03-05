pub use super::disk::*;
pub use super::validate::*;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Debug, PartialEq)]
pub enum FSTabMode {
    UUID,
    Label,
    Device,
}
pub const FSTAB_MODES: &'static [&'static str] = &["uuid", "label", "device"];
//When changing this, remember to change match_fstab_mode()

#[derive(Deserialize, Serialize, Debug)]
pub struct EnvConf {
    pub chrootcmd: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallationConf {
    pub pkglisturl: Option<String>,
    pub packages: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FSTabConf {
    pub mode: FSTabMode,
    pub path: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SystemdConf {
    pub enable_units: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TimeConf {
    pub timezone: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LangConf {
    pub default: Option<String>,
    pub locales: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SeedConf {
    #[serde(default = "seed_default_workdir")]
    pub workdir: String,

    pub env: EnvConf,
    pub disks: Vec<DiskConf>,
    pub installation: InstallationConf,
    pub fstab: FSTabConf,
    pub systemd: SystemdConf,
    pub symlinks: Option<Vec<HashMap<String, String>>>,
    pub time: Option<TimeConf>,
    pub lang: Option<LangConf>,
}

impl Validate for SeedConf {
    fn validate(&mut self) -> Result<(), ValidationError> {
        //Validate all disks
        for disk in &mut self.disks {
            disk.validate()?;
        }

        Ok(())
    }
}

impl Drop for SeedConf {
    fn drop(&mut self) {
        match self.unmount_partitions() {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to unmount remaining partitions! The system may be in a uncontrolled state! (error: {})", e.to_string());
            }
        }
    }
}

///	Matches a string of the fstab mode value to the correct FSTabMode
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The FSTabMode, else the string that was not matched
fn match_fstab_mode(value: &str) -> Result<FSTabMode, &str> {
    match value {
        "uuid" => Ok(FSTabMode::UUID),
        "label" => Ok(FSTabMode::Label),
        "device" => Ok(FSTabMode::Device),
        _ => Err(value),
    }
}

/// Returns the default value for the working directory
/// # Returns
/// Default workdir for seed: "./seed_workdir/"
fn seed_default_workdir() -> String{
    "./seed_workdir/".to_owned()
}

//
//	A custom deserializer for FSTabMode
//
impl<'de> Deserialize<'de> for FSTabMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FSTabModeVisitor)
    }
}
struct FSTabModeVisitor;
impl<'de> de::Visitor<'de> for FSTabModeVisitor {
    type Value = FSTabMode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a case-insensitive variant of FSTabMode")
    }

    fn visit_str<E>(self, value: &str) -> Result<FSTabMode, E>
    where
        E: de::Error,
    {
        match match_fstab_mode(value.to_lowercase().as_str()) {
            Ok(s) => Ok(s),
            Err(v) => Err(de::Error::custom(format!(
                "Invalid variant of FSTabMode {v}, expected one of {FSTAB_MODES:?}"
            ))),
        }
    }
}
