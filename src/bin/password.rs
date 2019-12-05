use structopt::StructOpt;
use std::io::{self};

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let mut count = 0;

    'outer: for c0 in 0..10 {
        for c1 in c0..10 {
            for c2 in c1..10 {
                for c3 in c2..10 {
                    for c4 in c3..10 {
                        for c5 in c4..10 {
                            let n = c0 * 100000
                                  + c1 * 10000
                                  + c2 * 1000
                                  + c3 * 100
                                  + c4 * 10
                                  + c5;
                            if n > args.to { break 'outer }
                            let doubles = (c0 == c1 && c1 != c2)
                                || (c1 == c2 && c0 != c1 && c1 != c3)
                                || (c2 == c3 && c1 != c2 && c3 != c4)
                                || (c3 == c4 && c2 != c3 && c4 != c5)
                                || (c4 == c5 && c3 != c4);
                            if doubles && n >= args.from {
                                count+=1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
    Ok(())
}

#[derive(StructOpt)]
struct Cli {
    from: u64,
    to: u64,
}
