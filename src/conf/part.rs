
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Visitor};
use std::fmt;
use super::validate::{Validate, ValidationError};
pub use super::size::*;

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
    Resize,
    Create,
}
const PART_ACTIONS: &'static [&'static str] = &["keep", "format", "resize", "create"];

///	Matches a string of the partition action value to the correct PartAction
/// # Arguments
/// * `value` - The value to match
/// # Returns
/// The PartAction, else the string that was not matched
fn match_part_action(value: &str) -> Result<PartAction, &str> {
    match value {
        "keep" => Ok(PartAction::Keep),
        "format" => Ok(PartAction::Format),
        "resize" => Ok(PartAction::Resize),
        "create" => Ok(PartAction::Create),
        _ => Err(value),
    }
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
                    warn!("Partition {}: Ignoring 'size': Not allowed in this mode", self.index);
                }
                if self.fs.is_some() {
                    warn!("Partition {}: Ignoring 'fs': Not allowed in this mode", self.index);
                }
            }
            //When formatting, changing the filesystem is allowed, but the size remains the same
            PartAction::Format => {
                if self.size.is_some() {
                    warn!("Partition {}: Ignoring 'size': Not allowed in this mode", self.index);
                }
                if self.fs.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'fs' is required when partition action is set to 'format'",
                    ));
                }
            }
            //When resizing, a size is required, but fs can't be changed
            PartAction::Resize => {
                if self.size.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'size' is required when action is 'resize'",
                    ));
                }
                if self.fs.is_some() {
                    warn!("Partition {}: Ignoring 'gs': Not allowed in this mode", self.index);
                }
            }
            //When creating, size is needed, fs only when mounted
            PartAction::Create => {
                if self.size.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'size' is required when action is 'create'",
                    ));
                }
                if self.mount.is_some() && self.fs.is_none() {
                    return Err(ValidationError::new(
                        self.index.to_string().as_str(),
                        "'fs' is required when new partition is mounted",
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
