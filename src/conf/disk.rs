
use serde::{Serialize, Deserialize, Deserializer, de};
use std::fmt;
pub use super::part::*;
use super::validate::*;

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
                    warn!("{}: Ignoring 'table': Not allowed in this mode", self.path);
                }
            },
            DiskAction::New => {
                if self.table.is_none() {
                    return Err(ValidationError::new(
                        self.path.as_str(),
                        "'table' field is required when disk action is 'new'",
                    ));
                }
            }
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
            part.index = index + 1;
            part.validate()?
        }

        Ok(())
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
impl<'de> de::Visitor<'de> for DiskActionVisitor {
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
