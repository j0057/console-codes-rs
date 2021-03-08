#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrivateMode {
    /// Cursor keys ESC 0 prefix instead of ESC [
    DECCKM = 1,

    /// 80/132 col mode switch
    DECCOLM = 3,

    /// Set reverse video mode
    DECSCNM = 5,

    /// Set cursor addressing relative to upper left corner of scrolling region
    DECOM = 6,

    /// Set autowrap on
    DECAWM = 7,

    /// Set keyboard autorepeat on
    DECARM = 8,

    /// X10 Mouse reporting mode 1 or reset to 0
    X10MR1 = 9,

    /// Make cursor visible
    DECTECM = 25,

    /// X11 Mouse reporting mode 2 or reset to 0
    X11MR2 = 1000,
}

impl From<PrivateMode> for u16 {
    fn from(pm: PrivateMode) -> Self {
        pm as Self
    }
}
