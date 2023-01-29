use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::{error::Error, fmt};

pub trait Validate {
    fn validate(&mut self) -> Result<(), ValidationError>;
}

#[derive(Debug)]
pub struct ValidationError {
    context: String,
    msg: String,
}
impl ValidationError {
    fn new(context: &str, message: &str) -> ValidationError {
        ValidationError {
            context: context.to_owned(),
            msg: message.to_owned(),
        }
    }
}
impl Error for ValidationError {}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed validation: {}: {}", self.context, self.msg)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DataSize {
    B,
    KB,
    MB,
    GB,
    TB,
}
const DATA_SIZES: &'static [&'static str] = &["B", "KB", "MB", "GB", "TB"];

///	Matches a string of the data size value to the correct DataSize
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The DataSize, else the string that was not matched
fn match_data_size(value: &str) -> Result<DataSize, &str> {
    match value {
        "b" => Ok(DataSize::B),
        "kb" => Ok(DataSize::KB),
        "mb" => Ok(DataSize::MB),
        "gb" => Ok(DataSize::GB),
        "tb" => Ok(DataSize::TB),
        _ => Err(value),
    }
}

pub fn data_size_bytes(count: i64, unit: &DataSize) -> i64 {
    match unit {
        DataSize::B => count,
        DataSize::KB => count * 1024,
        DataSize::MB => count * 1024 * 1024,
        DataSize::GB => count * 1024 * 1024 * 1024,
        DataSize::TB => count * 1024 * 1024 * 1024 * 1024,
    }
}

#[derive(Serialize, Debug)]
pub enum PartSize {
    Size(u32, DataSize),
    PercentTotal(f32),
    PercentFree(f32),
    Total(PartTotal),
}
const PART_SIZES: &'static [&'static str] = &["%", "%%", "min/max..."];

#[derive(Serialize, Debug)]
pub enum PartTotal {
    Min,
    Max,
}
const PART_TOTALS: &'static [&'static str] = &["min", "max"];

///	Matches a string of the partition total value to the correct PartTotal
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The PartTotal, else the string that was not matched
fn match_part_total(value: &str) -> Result<PartTotal, &str> {
    match value {
        "min" => Ok(PartTotal::Min),
        "max" => Ok(PartTotal::Max),
        _ => Err(value),
    }
}

/// The possible actions that can be performed on a partition
#[derive(Serialize, Debug, PartialEq)]
pub enum PartAction {
    Keep,
    Format,
    Shrink,
    Create,
}
const PART_ACTIONS: &'static [&'static str] = &["keep", "format", "shrink", "create"];

///	Matches a string of the partition action value to the correct PartAction
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The PartAction, else the string that was not matched
fn match_part_action(value: &str) -> Result<PartAction, &str> {
    match value {
        "keep" => Ok(PartAction::Keep),
        "format" => Ok(PartAction::Format),
        "shrink" => Ok(PartAction::Shrink),
        "create" => Ok(PartAction::Create),
        _ => Err(value),
    }
}

/// The possible actions that can be performed on a disk
#[derive(Serialize, Debug, PartialEq)]
pub enum DiskAction {
    Locked,
    Keep,
    Alter,
    New,
}
const DISK_ACTIONS: &'static [&'static str] = &["locked", "keep", "alter", "new"];

///	Matches a string of the disk action value to the correct DiskAction
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The DiskAction, else the string that was not matched
fn match_disk_action(value: &str) -> Result<DiskAction, &str> {
    match value {
        "locked" => Ok(DiskAction::Locked),
        "keep" => Ok(DiskAction::Keep),
        "alter" => Ok(DiskAction::Alter),
        "new" => Ok(DiskAction::New),
        _ => Err(value),
    }
}

/// The possible fstab modes
#[derive(Serialize, Debug, PartialEq)]
pub enum FSTabMode {
    UUID,
    Label,
    Device,
}
const FSTAB_MODES: &'static [&'static str] = &["uuid", "label", "device"];

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

#[derive(Deserialize, Serialize, Debug)]
pub struct EnvConf {
    pub chrootcmd: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PartConf {
	#[serde(default)]
	pub index: usize,
    pub action: PartAction,
    pub size: Option<PartSize>,
    pub fs: Option<String>,
    pub mount: Option<String>,
    pub fstab: Option<bool>,
}

impl Validate for PartConf {
    fn validate(&mut self) -> Result<(), ValidationError> {
        //Match the actions with their allowed entries
        match self.action {
            //When keeping, the size can't be altered and the fs can't be changed
            PartAction::Keep => {
                if self.size.is_some() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'size' not allowed when partition action is set to 'keep'",
                    ));
                }
                if self.fs.is_some() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'fs' not allowed when partition action is set to 'keep'",
                    ));
                }
            }
            //When formatting, changing the filesystem is allowed, but the size remains the same
            PartAction::Format => {
                if self.size.is_some() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'size' not allowed when partition action is set to 'format'",
                    ));
                }
                if self.fs.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'fs' required when partition action is set to 'format'",
                    ));
                }
            }
            //When shrinking, a size is required, but fs can't be changed
            PartAction::Shrink => {
                if self.size.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'size' required when action is 'shrink'",
                    ));
                }
                if self.fs.is_some() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'fs' not allowed when partition action is set to 'shrink'",
                    ));
                }
            }
            //When creating, size is needed, fs only when mounted
            PartAction::Create => {
                if self.size.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'size' required when action is 'create'",
                    ));
                }
                if self.mount.is_some() && self.fs.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'fs' required when 'new' partition is mounted",
                    ));
                }
            }
        }

        //The fstab needs a mount point
        if self.mount.is_none() {
            match self.fstab {
                Some(t) => {
                    if t {
                        return Err(ValidationError::new(
                            self.index.to_string().as_str(),
                            "Can not add partition without 'mount' to fstab",
                        ));
                    }
                }
                None => (),
            }
        }

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiskConf {
    pub path: String,
    pub action: DiskAction,
    pub table: Option<String>,
    pub partitions: Vec<PartConf>,
}

impl Validate for DiskConf {
    fn validate(&mut self) -> Result<(), ValidationError> {
        //The partition table can only be altered in 'new' mode
        match self.action {
            DiskAction::Locked | DiskAction::Keep | DiskAction::Alter => {
                if self.table.is_some() {
                    return Err(ValidationError::new(
                        self.path.as_str(),
                        "Can only alter partition table in 'new' mode",
                    ));
                }
            }
            _ => (),
        }

        //If we are in 'locked' mode, do not allow any modifications to partitions
        if self.action == DiskAction::Locked {
            for part in &self.partitions {

                if part.action != PartAction::Keep {
                    let context = format!("{} -> {}", self.path, part.index.to_string().as_str());
                    return Err(ValidationError::new(
                        context.as_str(),
                        "Partition action must be 'keep' if disk action is 'locked'",
                    ));
                }
            }
        }

        //Then validate the partitions
        for (index, part) in self.partitions.iter_mut().enumerate() {
			part.index = index;
			part.validate()?
        }

        Ok(())
    }
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

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallFile {
    pub seed: SeedConf,
}

impl Validate for InstallFile {
    fn validate(&mut self) -> Result<(), ValidationError> {
        self.seed.validate()
    }
}

//
//	A custom deserializer for PartSize
//

impl<'de> Deserialize<'de> for PartSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PartSizeVisitor)
    }
}
struct PartSizeVisitor;
impl<'de> Visitor<'de> for PartSizeVisitor {
    type Value = PartSize;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a case-insensitive variant of PartSize")
    }

    fn visit_str<E>(self, value: &str) -> Result<PartSize, E>
    where
        E: de::Error,
    {
        let (num_str, rest) = value.split_at(match value.find(|c: char| !c.is_numeric()) {
            Some(s) => s,
            None => {
                return Err(de::Error::custom(format!(
                "Size tag can't be empty / must have a unit of {PART_SIZES:?} or {DATA_SIZES:?}"
            )))
            }
        });

        //If this is a total (Min, Max)
        if num_str == "" {
            match match_part_total(rest) {
                Ok(o) => return Ok(PartSize::Total(o)),
                Err(e) => {
                    return Err(de::Error::custom(format!(
                        "Invalid variant of PartTotal {e}, expected one of {PART_TOTALS:?}"
                    )))
                }
            }
        }

        match rest {
            "%" => {
                return Ok(PartSize::PercentTotal(
                    num_str.parse::<f32>().unwrap() / 100.0,
                ))
            }
            "%%" => {
                return Ok(PartSize::PercentFree(
                    num_str.parse::<f32>().unwrap() / 100.0,
                ))
            }
            other => match match_data_size(other.to_lowercase().as_str()) {
                Ok(s) => return Ok(PartSize::Size(num_str.parse().unwrap(), s)),
                Err(e) => {
                    return Err(de::Error::custom(format!(
                        "Invalid variant of PartSize {}, expected one of {:?} or {:?}",
                        e, PART_SIZES, DATA_SIZES
                    )))
                }
            },
        }
    }
}

//
//	A custom deserializer for PartAction
//
impl<'de> Deserialize<'de> for PartAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PartActionVisitor)
    }
}
struct PartActionVisitor;
impl<'de> Visitor<'de> for PartActionVisitor {
    type Value = PartAction;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a case-insensitive variant of PartAction")
    }

    fn visit_str<E>(self, value: &str) -> Result<PartAction, E>
    where
        E: de::Error,
    {
        match match_part_action(value.to_lowercase().as_str()) {
            Ok(s) => Ok(s),
            Err(v) => Err(de::Error::custom(format!(
                "Invalid variant of PartAction {v}, expected one of {PART_ACTIONS:?}"
            ))),
        }
    }
}

//
//	A custom deserializer for DiskAction
//
impl<'de> Deserialize<'de> for DiskAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DiskActionVisitor)
    }
}
struct DiskActionVisitor;
impl<'de> Visitor<'de> for DiskActionVisitor {
    type Value = DiskAction;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a case-insensitive variant of DiskAction")
    }

    fn visit_str<E>(self, value: &str) -> Result<DiskAction, E>
    where
        E: de::Error,
    {
        match match_disk_action(value.to_lowercase().as_str()) {
            Ok(s) => Ok(s),
            Err(v) => Err(de::Error::custom(format!(
                "Invalid variant of DiskAction {v}, expected one of {DISK_ACTIONS:?}"
            ))),
        }
    }
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
impl<'de> Visitor<'de> for FSTabModeVisitor {
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
