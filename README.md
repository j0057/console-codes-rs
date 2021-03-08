# console-codes-rs

Some enums for rendering escape sequences as documented in
[console\_codes(4)][1]. The main enum CSI implements Display so that it can
easily be converted to a String.

[1]: https://man.archlinux.org/man/console_codes.4

## Example

Use the `Erase Display` code to clear the screen, the `CUrsor Position` code to
move the cursor to the top left corner, then a `Set Graphics Rendition`
sequence with three attributes to set the colors to blinking blue text on deep
sky blue background and print _Hello, world!_:

    use console_codes::{CSI, SGR};
    
    fn main() {
        print!("{}", CSI::ED(2));
        print!("{}", CSI::CUP(1, 1));
        print!("{}", CSI::SGR(&[SGR::Blink, SGR::FG24(0, 0, 255), SGR::BG24(0, 191, 255)]));
        print!("{}", "Hello, world!");
        print!("{}", CSI::SGR(&[SGR::Reset]));
        println!();
    }
