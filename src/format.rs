/// Output format understood by Space‑Track.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Json,
    Csv,
    Xml,
    Html,
    Tle,
    ThreeLine,
    Kvn,
}

impl Format {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Xml => "xml",
            Self::Html => "html",
            Self::Tle => "tle",
            Self::ThreeLine => "3le",
            Self::Kvn => "kvn",
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Self::Json
    }
}
