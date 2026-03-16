use super::icn_names::ICON_NAMES;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IcnName {
    Named(String),
    Indexed(usize),
}

impl IcnName {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Named(name) => name.as_str(),
            Self::Indexed(index) => ICON_NAMES
                .get(*index)
                .copied()
                .unwrap_or_else(|| panic!("icon index {index} is out of bounds")),
        }
    }
}

impl From<String> for IcnName {
    fn from(value: String) -> Self {
        Self::Named(value)
    }
}

impl From<&str> for IcnName {
    fn from(value: &str) -> Self {
        Self::Named(value.to_owned())
    }
}

impl From<usize> for IcnName {
    fn from(value: usize) -> Self {
        Self::Indexed(value)
    }
}
