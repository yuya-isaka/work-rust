use std::env;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let lines: usize = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            std::process::exit(1);
        }
    };

    if let Err(e) = head(io::stdin().lock(), lines) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
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
