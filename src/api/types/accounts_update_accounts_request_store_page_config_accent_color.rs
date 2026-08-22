pub use crate::prelude::*;

/// Accent color used on the account store page.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAccountsRequestStorePageConfigAccentColor {
    Ruby,
    Tomato,
    Red,
    Crimson,
    Pink,
    Plum,
    Purple,
    Violet,
    Iris,
    Cyan,
    Teal,
    Jade,
    Green,
    Grass,
    Brown,
    Blue,
    Orange,
    Indigo,
    Sky,
    Mint,
    Yellow,
    Amber,
    Lime,
    Lemon,
    Magenta,
    Gold,
    Bronze,
    Gray,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAccountsRequestStorePageConfigAccentColor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ruby => serializer.serialize_str("ruby"),
            Self::Tomato => serializer.serialize_str("tomato"),
            Self::Red => serializer.serialize_str("red"),
            Self::Crimson => serializer.serialize_str("crimson"),
            Self::Pink => serializer.serialize_str("pink"),
            Self::Plum => serializer.serialize_str("plum"),
            Self::Purple => serializer.serialize_str("purple"),
            Self::Violet => serializer.serialize_str("violet"),
            Self::Iris => serializer.serialize_str("iris"),
            Self::Cyan => serializer.serialize_str("cyan"),
            Self::Teal => serializer.serialize_str("teal"),
            Self::Jade => serializer.serialize_str("jade"),
            Self::Green => serializer.serialize_str("green"),
            Self::Grass => serializer.serialize_str("grass"),
            Self::Brown => serializer.serialize_str("brown"),
            Self::Blue => serializer.serialize_str("blue"),
            Self::Orange => serializer.serialize_str("orange"),
            Self::Indigo => serializer.serialize_str("indigo"),
            Self::Sky => serializer.serialize_str("sky"),
            Self::Mint => serializer.serialize_str("mint"),
            Self::Yellow => serializer.serialize_str("yellow"),
            Self::Amber => serializer.serialize_str("amber"),
            Self::Lime => serializer.serialize_str("lime"),
            Self::Lemon => serializer.serialize_str("lemon"),
            Self::Magenta => serializer.serialize_str("magenta"),
            Self::Gold => serializer.serialize_str("gold"),
            Self::Bronze => serializer.serialize_str("bronze"),
            Self::Gray => serializer.serialize_str("gray"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAccountsRequestStorePageConfigAccentColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ruby" => Ok(Self::Ruby),
            "tomato" => Ok(Self::Tomato),
            "red" => Ok(Self::Red),
            "crimson" => Ok(Self::Crimson),
            "pink" => Ok(Self::Pink),
            "plum" => Ok(Self::Plum),
            "purple" => Ok(Self::Purple),
            "violet" => Ok(Self::Violet),
            "iris" => Ok(Self::Iris),
            "cyan" => Ok(Self::Cyan),
            "teal" => Ok(Self::Teal),
            "jade" => Ok(Self::Jade),
            "green" => Ok(Self::Green),
            "grass" => Ok(Self::Grass),
            "brown" => Ok(Self::Brown),
            "blue" => Ok(Self::Blue),
            "orange" => Ok(Self::Orange),
            "indigo" => Ok(Self::Indigo),
            "sky" => Ok(Self::Sky),
            "mint" => Ok(Self::Mint),
            "yellow" => Ok(Self::Yellow),
            "amber" => Ok(Self::Amber),
            "lime" => Ok(Self::Lime),
            "lemon" => Ok(Self::Lemon),
            "magenta" => Ok(Self::Magenta),
            "gold" => Ok(Self::Gold),
            "bronze" => Ok(Self::Bronze),
            "gray" => Ok(Self::Gray),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAccountsRequestStorePageConfigAccentColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ruby => write!(f, "ruby"),
            Self::Tomato => write!(f, "tomato"),
            Self::Red => write!(f, "red"),
            Self::Crimson => write!(f, "crimson"),
            Self::Pink => write!(f, "pink"),
            Self::Plum => write!(f, "plum"),
            Self::Purple => write!(f, "purple"),
            Self::Violet => write!(f, "violet"),
            Self::Iris => write!(f, "iris"),
            Self::Cyan => write!(f, "cyan"),
            Self::Teal => write!(f, "teal"),
            Self::Jade => write!(f, "jade"),
            Self::Green => write!(f, "green"),
            Self::Grass => write!(f, "grass"),
            Self::Brown => write!(f, "brown"),
            Self::Blue => write!(f, "blue"),
            Self::Orange => write!(f, "orange"),
            Self::Indigo => write!(f, "indigo"),
            Self::Sky => write!(f, "sky"),
            Self::Mint => write!(f, "mint"),
            Self::Yellow => write!(f, "yellow"),
            Self::Amber => write!(f, "amber"),
            Self::Lime => write!(f, "lime"),
            Self::Lemon => write!(f, "lemon"),
            Self::Magenta => write!(f, "magenta"),
            Self::Gold => write!(f, "gold"),
            Self::Bronze => write!(f, "bronze"),
            Self::Gray => write!(f, "gray"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
