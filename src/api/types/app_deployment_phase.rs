pub use crate::prelude::*;

/// The stage a running deployment has reached, or `null` when none is running. Later phases dominate the wall clock: `process_archive` waits on the upload pipeline and `promote` waits for the build to go live.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppDeploymentPhase {
    Install,
    Build,
    Typecheck,
    UploadBuild,
    UploadSource,
    ProcessArchive,
    CreateBuild,
    Promote,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AppDeploymentPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Install => serializer.serialize_str("install"),
            Self::Build => serializer.serialize_str("build"),
            Self::Typecheck => serializer.serialize_str("typecheck"),
            Self::UploadBuild => serializer.serialize_str("upload_build"),
            Self::UploadSource => serializer.serialize_str("upload_source"),
            Self::ProcessArchive => serializer.serialize_str("process_archive"),
            Self::CreateBuild => serializer.serialize_str("create_build"),
            Self::Promote => serializer.serialize_str("promote"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AppDeploymentPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "install" => Ok(Self::Install),
            "build" => Ok(Self::Build),
            "typecheck" => Ok(Self::Typecheck),
            "upload_build" => Ok(Self::UploadBuild),
            "upload_source" => Ok(Self::UploadSource),
            "process_archive" => Ok(Self::ProcessArchive),
            "create_build" => Ok(Self::CreateBuild),
            "promote" => Ok(Self::Promote),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AppDeploymentPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Install => write!(f, "install"),
            Self::Build => write!(f, "build"),
            Self::Typecheck => write!(f, "typecheck"),
            Self::UploadBuild => write!(f, "upload_build"),
            Self::UploadSource => write!(f, "upload_source"),
            Self::ProcessArchive => write!(f, "process_archive"),
            Self::CreateBuild => write!(f, "create_build"),
            Self::Promote => write!(f, "promote"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
