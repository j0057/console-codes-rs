#[derive(Debug, Clone, Copy)]
pub enum Mode {
    /// Display control chars
    DECCRM = 3,

    /// Set insert mode
    DECIM = 4,

    /// Automatically follow echo of LF, VT or FF with CR
    LFNL = 20,
}

impl From<Mode> for u16 {
    fn from(m: Mode) -> Self {
        m as Self
    }
}
