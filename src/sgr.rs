use crate::color::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SGR {
    Reset,
    Bold,
    HalfBright,
    Underscore,
    Blink,
    Reverse,
    Underline,
    NormalIntensity,
    UnderlineOff,
    BlinkOff,
    ReverseOff,

    /// Set foreground color
    FG(Color),

    /// Set one of 256 colors; 0..15 are IGBR (black, red, green, ... white); 16..231 a 6x6x6 color
    /// cube, 232..255 a grayscale ramp
    FG256(u8),

    /// Set 24-bit color, r/g/b components are in the range 0..255
    FG24(u8, u8, u8),

    DefaultFG,

    /// Set background color
    BG(Color),

    BG256(u8),

    BG24(u8, u8, u8),

    DefaultBG,

    /// Set foreground to bright versions of SGRColor
    FGB(Color),

    /// Set background color, same as BG (bright not supported)
    BGB(Color),
}

impl Into<Vec<u16>> for SGR {
    fn into(self) -> Vec<u16> {
        use SGR::*;
        match self {
            Reset           => vec![0],
            Bold            => vec![1],
            HalfBright      => vec![2],
            Underscore      => vec![4],
            Blink           => vec![5],
            Reverse         => vec![7],
            Underline       => vec![21],
            NormalIntensity => vec![22],
            UnderlineOff    => vec![24],
            BlinkOff        => vec![25],
            ReverseOff      => vec![27],
            FG(n)           => vec![30 + u16::from(n)],
            FG256(n)        => vec![38, 5, n.into()],
            FG24(r, g, b)   => vec![38, 2, r.into(), g.into(), b.into()],
            DefaultFG       => vec![39],
            BG(n)           => vec![40 + u16::from(n)],
            BG256(n)        => vec![48, 5, n.into()],
            BG24(r, g, b)   => vec![48, 2, r.into(), g.into(), b.into()],
            DefaultBG       => vec![49],
            FGB(n)          => vec![90 + u16::from(n)],
            BGB(n)          => vec![100 + u16::from(n)],
        }
    }
}
