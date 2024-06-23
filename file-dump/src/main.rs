use std::env;
use std::fs;
use std::io::{self, Read};
const BYTES_LINE: usize = 16;

// io::Result<()>
//                           ... io::Result型の値を返す
// env::args().nth(1)
//                           ... コマンドライン引数を取得
// fs::File::open()
//                           ... ファイルを開く
// file.read_exact(&mut buf)
//                           ... バッファに読み込む
// print!()
//                           ... 標準出力
// is_ascii_alphanumeric()
//                           ... ASCIIの英数字かどうか
// expect()
//                           ... Option型の値がNoneだった場合にエラーメッセージを表示
// is_ok()
//                           ... Result型の値がOkだった場合にtrueを返す

fn main() -> io::Result<()> {
    let arg = env::args().nth(1).expect("no argument");
    let mut file = fs::File::open(arg).expect("cannot open file");

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
