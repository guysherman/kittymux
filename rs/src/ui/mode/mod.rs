pub mod navigatemode;
pub mod quicknavmode;
pub mod renamemode;
pub mod setquicknavmode;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Navigate,
    Rename,
    SetQuickNav,
    QuickNav,
}
