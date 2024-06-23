use std::io::Read;
const BYTES_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg = std::env::args().nth(1).expect("no argument");
    let mut file = std::fs::File::open(arg).expect("cannot open file");

    let mut pos = 0;
    let mut buf = [0; BYTES_LINE];

    println!();
    while file.read_exact(&mut buf).is_ok() {
        print!("0x{:08x}  ", pos);
        for b in buf {
            print!("{:02x} ", b);
        }
        for _ in 0..BYTES_LINE - buf.len() {
            print!("   ");
        }
        print!(" ");
        for b in buf {
            print!(
                "{}",
                if b.is_ascii_alphanumeric() {
                    b as char
                } else {
                    '.'
                }
            );
        }
        println!();
        pos += BYTES_LINE;
    }

    Ok(())
}
