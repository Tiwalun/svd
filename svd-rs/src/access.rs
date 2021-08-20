/// Defines access rights for fields on the device, though it may be specified at a
/// higher level than individual fields.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Access {
    /// Read access is permitted. Write operations have an undefined effect.
    #[cfg_attr(feature = "serde", serde(rename = "read-only"))]
    ReadOnly,

    /// Read and write accesses are permitted.
    #[cfg_attr(feature = "serde", serde(rename = "read-write"))]
    ReadWrite,

    /// Read access is always permitted.
    /// Only the first write after a reset will affect the content.
    /// Following writes have an undefined effect.
    #[cfg_attr(feature = "serde", serde(rename = "read-writeOnce"))]
    ReadWriteOnce,

    /// Read operations have undefined results.
    /// Only the first write after a reset will affect the content.
    #[cfg_attr(feature = "serde", serde(rename = "writeOnce"))]
    WriteOnce,

    /// Read operations have an undefined result. Write access is permitted.
    #[cfg_attr(feature = "serde", serde(rename = "write-only"))]
    WriteOnly,
}

impl Access {
    /// Whether the register/field is readable at least once.
    pub fn can_read(self) -> bool {
        match self {
            Self::ReadOnly | Self::ReadWrite | Self::ReadWriteOnce => true,
            _ => false,
        }
    }

    /// Whether the register/field is writable at least once.
    pub fn can_write(self) -> bool {
        match self {
            Self::ReadOnly => false,
            _ => true,
        }
    }
}

impl Default for Access {
    fn default() -> Self {
        Self::ReadWrite
    }
}

impl Access {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "read-only" => Some(Access::ReadOnly),
            "read-write" => Some(Access::ReadWrite),
            "read-writeOnce" => Some(Access::ReadWriteOnce),
            "write-only" => Some(Access::WriteOnly),
            "writeOnce" => Some(Access::WriteOnce),
            _ => None,
        }
    }
}
