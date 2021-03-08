use std::io::Write;

use console_codes::{CSI, SGR};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();

    for x in (0x00..=0xff).step_by(0x0f) {
        //let y = ((0xff - x) as f64 * 0.4) as u8;
        let y = 0xff  - x;
        writeln!(stdout,
                 "{}red: {:>3}{} green: {:>3}{} blue: {:>3}",
                 CSI::SGR(&[SGR::BG24(y, 0, y), SGR::FG24(x, 0, 0)]), x,
                 CSI::SGR(&[SGR::BG24(y, y, 0), SGR::FG24(0, x, 0)]), x,
                 CSI::SGR(&[SGR::BG24(0, y, y), SGR::FG24(0, 0, x)]), x)?;
    }

    Ok(())
}
