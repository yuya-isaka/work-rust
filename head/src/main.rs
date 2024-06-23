use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process;

// io::Result<()>
//                           ... io::Result型の値を返す
// env::args().collect()
//                           ... コマンドライン引数をVec<String>に変換
// Vec<String>
//                           ... String型の可変長配列
// args.len()
//                           ... Vecの長さ
// eprintln!()
//                           ... 標準エラー出力
// process::exit(1)
//                           ... プログラムを終了
// match args[1].parse()
//                           ... 文字列を数値に変換
// Box<dyn BufRead>
//                           ... 動的ディスパッチ, 異なる型の値を同じインターフェースで扱う
// BufRead
//                           ... バッファリングされた入力ストリームからの読み込みを提供するトレイト
// BufReader::new()
//                           ... バッファリングされた入力ストリームを作成
// File::open()
//                           ... ファイルを開く
// io::stdin()
//                           ... 標準入力
// Box::new()
//                           ... ヒープに確保
// reader.lines()
//                           ... Readerから1行ずつ読み込む

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: {} n [file]", args[0]);
        process::exit(1);
    }

    let lines: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            process::exit(1);
        }
    };

    let reader: Box<dyn BufRead> = if args.len() == 3 {
        let filename = &args[2];
        let file = File::open(filename)?;
        Box::new(BufReader::new(file))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    head(reader, lines)
}

fn head<R: BufRead>(reader: R, mut lines: usize) -> io::Result<()> {
    if lines == 0 {
        return Ok(());
    }

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        lines -= 1;
        if lines == 0 {
            break;
        }
    }

    Ok(())
}
