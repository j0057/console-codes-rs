use console_codes::{CSI, SGR};

fn main() {
    print!("{}", CSI::ED(2));
    print!("{}", CSI::CUP(1, 1));
    print!("{}", CSI::SGR(&[SGR::Blink, SGR::FG24(0, 0, 255), SGR::BG24(0, 191, 255)]));
    print!("{}", "Hello, world!");
    print!("{}", CSI::SGR(&[SGR::Reset]));
    println!();
}
