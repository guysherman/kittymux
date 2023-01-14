use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum EntryType {
    OsWindow,
    Tab,
    Window,
}
