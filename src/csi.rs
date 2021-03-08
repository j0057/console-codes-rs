use crate::mode::Mode;
use crate::privmode::PrivateMode;
use crate::color::Color;
use crate::sgr::SGR;

#[derive(Debug, Eq, PartialEq)]
pub enum CSI<'a> {
    /// CUrsor Up
    CUU(u16),

    /// CUrsor Down
    CUD(u16),

    /// CUrsor Forward
    CUF(u16),

    /// CUrsor Backward
    CUB(u16),

    /// CUrsor Next line
    CNL(u16),

    /// CUrsor Preceding line
    CPL(u16),

    /// Cursor cHaracter Absolute
    CHA(u16),

    /// CUrsor Position
    CUP(u16, u16),

    /// Erase Display; 0: from cursor to end of display; 1: from start to cursor; 2: whole display;
    /// 3: whole display including scrollback
    ED(u16),

    /// Erase Line; 0: from cursor to end of line; 1: from start of line to cursor; 2: whole line
    EL(u16),

    /// Set Mode
    #[allow(dead_code)]
    SM(Mode),

    /// Reset Mode
    #[allow(dead_code)]
    RM(Mode),

    /// Private Set Mode
    #[allow(dead_code)]
    PSM(PrivateMode),

    /// Private Reset Mode
    #[allow(dead_code)]
    PRM(PrivateMode),

    /// Set Graphics Rendition
    SGR(&'a [SGR]),

    /// Set underline color
    LxULColor(Color),

    /// Set dim color
    LxDimColor(Color),

    /// Make the current color pair the default attributes
    LxDefColor,

    /// Set screen blank timeout to n minutes
    LxScreenBlankTimeout(u16),

    /// Set bell frequency in Hz
    LxBellFrequency(u16),

    /// Set bell duration in msec
    LxBellDuration(u16),

    /// Bring specified console to the front
    LxActivateConsole(u8),

    /// Unblank the screen
    LxUnblankScreen,

    /// Set VESA powerdown interval in minutes
    LxVESAPowerdownInterval(u16),

    /// Bring the previous console to the front
    LxActivatePreviousConsole,

    /// Set cursor blink interval in milliseconds
    LxSetCursorBlinkInterval(u16),
}

fn csi(attr: &[u16], ch: char) -> String {
    format!("\x1b[{}{}", attr.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(";"), ch)
}

fn decpm(p: u16, ch: char) -> String {
    format!("\x1b[?{}{}", p, ch)
}

fn sgr(attr: &[SGR]) -> Vec<u16> {
    attr.iter()
        .flat_map::<Vec<u16>, _>(|a| (*a).into())
        .collect()
}

impl<'a> std::fmt::Display for CSI<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match *self {
            CSI::CUU(n)                         => csi(&[n], 'A'),
            CSI::CUD(n)                         => csi(&[n], 'B'),
            CSI::CUF(n)                         => csi(&[n], 'C'),
            CSI::CUB(n)                         => csi(&[n], 'D'),
            CSI::CNL(n)                         => csi(&[n], 'E'),
            CSI::CPL(n)                         => csi(&[n], 'F'),
            CSI::CHA(n)                         => csi(&[n], 'G'),
            CSI::CUP(r, c)                      => csi(&[r, c], 'H'),
            CSI::ED(m) if m <= 3                => csi(&[m], 'J'),
            CSI::ED(x)                          => panic!("illegal ED mode {}", x),
            CSI::EL(m) if m <= 2                => csi(&[m], 'K'),
            CSI::EL(x)                          => panic!("illegal EL mode {}", x),
            CSI::SM(m)                          => csi(&[m.into()], 'h'),
            CSI::RM(m)                          => csi(&[m.into()], 'l'),
            CSI::PSM(m)                         => decpm(m.into(), 'h'),
            CSI::PRM(m)                         => decpm(m.into(), 'l'),
            CSI::SGR(attr)                      => csi(&sgr(attr), 'm'),
            CSI::LxULColor(c)                   => csi(&[1, c.into()], ']'),
            CSI::LxDimColor(c)                  => csi(&[2, c.into()], ']'),
            CSI::LxDefColor                     => csi(&[8], ']'),
            CSI::LxScreenBlankTimeout(n)        => csi(&[9, n], ']'),
            CSI::LxBellFrequency(n)             => csi(&[10, n], ']'),
            CSI::LxBellDuration(t)              => csi(&[11, t], ']'),
            CSI::LxActivateConsole(n)           => csi(&[12, n as u16], ']'),
            CSI::LxUnblankScreen                => csi(&[13], ']'),
            CSI::LxVESAPowerdownInterval(n)     => csi(&[14, n], ']'),
            CSI::LxActivatePreviousConsole      => csi(&[15], ']'),
            CSI::LxSetCursorBlinkInterval(n)    => csi(&[16, n], ']'),
        })
    }
}
