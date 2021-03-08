use console_codes::{CSI, Mode, PrivateMode, Color, SGR};

#[test]
fn test_csi() {
    [(CSI::CUU(1),      "\x1b[1A"),
     (CSI::CUD(2),      "\x1b[2B"),
     (CSI::CUF(3),      "\x1b[3C"),
     (CSI::CUB(4),      "\x1b[4D"),
     (CSI::CNL(5),      "\x1b[5E"),
     (CSI::CPL(6),      "\x1b[6F"),
     (CSI::CHA(7),      "\x1b[7G"),
     (CSI::CUP(13, 32), "\x1b[13;32H"),
     (CSI::ED(2),       "\x1b[2J"),
     (CSI::EL(2),       "\x1b[2K")]
     // L, M, P, X, a, b, c, d, e, f, g : not implemented
     // h, l -> test_mode / test_private_mode
     // m -> test_sgr
     // n, q, r, s, u, ` : not implemented
        .iter()
        .for_each(|(csi, exp)| assert_eq!(csi.to_string(), *exp));
}

#[test]
fn test_mode() {
    [(CSI::SM(Mode::DECCRM), "\x1b[3h"),
     (CSI::SM(Mode::DECIM),  "\x1b[4h"),
     (CSI::SM(Mode::LFNL),   "\x1b[20h"),
     (CSI::RM(Mode::DECCRM), "\x1b[3l"),
     (CSI::RM(Mode::DECIM),  "\x1b[4l"),
     (CSI::RM(Mode::LFNL),   "\x1b[20l")]
         .iter()
         .for_each(|(csi, exp)| assert_eq!(csi.to_string(), *exp));
}

#[test]
fn test_private_mode() {
    [(CSI::PSM(PrivateMode::DECCKM),  "\x1b[?1h"),
     (CSI::PSM(PrivateMode::DECCOLM), "\x1b[?3h"),
     (CSI::PSM(PrivateMode::DECSCNM), "\x1b[?5h"),
     (CSI::PSM(PrivateMode::DECOM),   "\x1b[?6h"),
     (CSI::PSM(PrivateMode::DECAWM),  "\x1b[?7h"),
     (CSI::PSM(PrivateMode::DECARM),  "\x1b[?8h"),
     (CSI::PSM(PrivateMode::X10MR1),  "\x1b[?9h"),
     (CSI::PSM(PrivateMode::DECTECM), "\x1b[?25h"),
     (CSI::PSM(PrivateMode::X11MR2),  "\x1b[?1000h"),
     (CSI::PRM(PrivateMode::DECCKM),  "\x1b[?1l"),
     (CSI::PRM(PrivateMode::DECCOLM), "\x1b[?3l"),
     (CSI::PRM(PrivateMode::DECSCNM), "\x1b[?5l"),
     (CSI::PRM(PrivateMode::DECOM),   "\x1b[?6l"),
     (CSI::PRM(PrivateMode::DECAWM),  "\x1b[?7l"),
     (CSI::PRM(PrivateMode::DECARM),  "\x1b[?8l"),
     (CSI::PRM(PrivateMode::X10MR1),  "\x1b[?9l"),
     (CSI::PRM(PrivateMode::DECTECM), "\x1b[?25l"),
     (CSI::PRM(PrivateMode::X11MR2),  "\x1b[?1000l")]
         .iter()
         .for_each(|(csi, exp)| assert_eq!(csi.to_string(), *exp));
}

#[test]
fn test_sgr() {
    [(CSI::SGR(&[SGR::Reset]),               "\x1b[0m"),
     (CSI::SGR(&[SGR::Bold]),                "\x1b[1m"),
     (CSI::SGR(&[SGR::HalfBright]),          "\x1b[2m"),
     (CSI::SGR(&[SGR::Underscore]),          "\x1b[4m"),
     (CSI::SGR(&[SGR::Blink]),               "\x1b[5m"),
     (CSI::SGR(&[SGR::Reverse]),             "\x1b[7m"),
     // 10, 11, 12 not implemented
     (CSI::SGR(&[SGR::Underline]),           "\x1b[21m"),
     (CSI::SGR(&[SGR::NormalIntensity]),     "\x1b[22m"),
     (CSI::SGR(&[SGR::UnderlineOff]),        "\x1b[24m"),
     (CSI::SGR(&[SGR::BlinkOff]),            "\x1b[25m"),
     (CSI::SGR(&[SGR::ReverseOff]),          "\x1b[27m"),

     (CSI::SGR(&[SGR::FG(Color::Black)]),    "\x1b[30m"),
     (CSI::SGR(&[SGR::FG(Color::Red)]),      "\x1b[31m"),
     (CSI::SGR(&[SGR::FG(Color::Green)]),    "\x1b[32m"),
     (CSI::SGR(&[SGR::FG(Color::Brown)]),    "\x1b[33m"),
     (CSI::SGR(&[SGR::FG(Color::Blue)]),     "\x1b[34m"),
     (CSI::SGR(&[SGR::FG(Color::Magenta)]),  "\x1b[35m"),
     (CSI::SGR(&[SGR::FG(Color::Cyan)]),     "\x1b[36m"),
     (CSI::SGR(&[SGR::FG(Color::White)]),    "\x1b[37m"),

     (CSI::SGR(&[SGR::FG256(23)]),           "\x1b[38;5;23m"),
     (CSI::SGR(&[SGR::FG24(29, 31, 37)]),    "\x1b[38;2;29;31;37m"),

     (CSI::SGR(&[SGR::DefaultFG]),           "\x1b[39m"),

     (CSI::SGR(&[SGR::BG(Color::Black)]),    "\x1b[40m"),
     (CSI::SGR(&[SGR::BG(Color::Red)]),      "\x1b[41m"),
     (CSI::SGR(&[SGR::BG(Color::Green)]),    "\x1b[42m"),
     (CSI::SGR(&[SGR::BG(Color::Brown)]),    "\x1b[43m"),
     (CSI::SGR(&[SGR::BG(Color::Blue)]),     "\x1b[44m"),
     (CSI::SGR(&[SGR::BG(Color::Magenta)]),  "\x1b[45m"),
     (CSI::SGR(&[SGR::BG(Color::Cyan)]),     "\x1b[46m"),
     (CSI::SGR(&[SGR::BG(Color::White)]),    "\x1b[47m"),

     (CSI::SGR(&[SGR::BG256(23)]),           "\x1b[48;5;23m"),
     (CSI::SGR(&[SGR::BG24(29, 31, 37)]),    "\x1b[48;2;29;31;37m"),

     (CSI::SGR(&[SGR::DefaultBG]),           "\x1b[49m"),

     (CSI::SGR(&[SGR::FGB(Color::Black)]),   "\x1b[90m"),
     (CSI::SGR(&[SGR::FGB(Color::Red)]),     "\x1b[91m"),
     (CSI::SGR(&[SGR::FGB(Color::Green)]),   "\x1b[92m"),
     (CSI::SGR(&[SGR::FGB(Color::Brown)]),   "\x1b[93m"),
     (CSI::SGR(&[SGR::FGB(Color::Blue)]),    "\x1b[94m"),
     (CSI::SGR(&[SGR::FGB(Color::Magenta)]), "\x1b[95m"),
     (CSI::SGR(&[SGR::FGB(Color::Cyan)]),    "\x1b[96m"),
     (CSI::SGR(&[SGR::FGB(Color::White)]),   "\x1b[97m"),

     (CSI::SGR(&[SGR::BGB(Color::Black)]),   "\x1b[100m"),
     (CSI::SGR(&[SGR::BGB(Color::Red)]),     "\x1b[101m"),
     (CSI::SGR(&[SGR::BGB(Color::Green)]),   "\x1b[102m"),
     (CSI::SGR(&[SGR::BGB(Color::Brown)]),   "\x1b[103m"),
     (CSI::SGR(&[SGR::BGB(Color::Blue)]),    "\x1b[104m"),
     (CSI::SGR(&[SGR::BGB(Color::Magenta)]), "\x1b[105m"),
     (CSI::SGR(&[SGR::BGB(Color::Cyan)]),    "\x1b[106m"),
     (CSI::SGR(&[SGR::BGB(Color::White)]),   "\x1b[107m")]
         .iter()
         .for_each(|(csi, exp)| assert_eq!(csi.to_string(), *exp));
}

#[test]
fn test_private_csi() {
    [(CSI::LxULColor(Color::Green),       "\x1b[1;2]"),
     (CSI::LxDimColor(Color::Brown),      "\x1b[2;3]"),
     (CSI::LxDefColor,                    "\x1b[8]"),
     (CSI::LxScreenBlankTimeout(10),      "\x1b[9;10]"),
     (CSI::LxBellFrequency(440),          "\x1b[10;440]"),
     (CSI::LxBellDuration(500),           "\x1b[11;500]"),
     (CSI::LxActivateConsole(3),          "\x1b[12;3]"),
     (CSI::LxUnblankScreen,               "\x1b[13]"),
     (CSI::LxVESAPowerdownInterval(10),   "\x1b[14;10]"),
     (CSI::LxActivatePreviousConsole,     "\x1b[15]"),
     (CSI::LxSetCursorBlinkInterval(100), "\x1b[16;100]")]
         .iter()
         .for_each(|(csi, exp)| assert_eq!(csi.to_string(), *exp));
}
