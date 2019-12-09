use std::sync::mpsc::channel;
use structopt::StructOpt;
use std::fs;
use std::io;
use aoc::IntCode;

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = fs::read_to_string(args.path)?;
    let initial: Vec<i32> = file
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    println!("part 1");
    let (send, rec) = channel();
    send.send(1).unwrap();
    IntCode::new(initial.clone()).run(&rec, &send);
    println!("{}", rec.recv().unwrap());

    println!("part 2");
    let (send, rec) = channel();
    send.send(5).unwrap();
    IntCode::new(initial.clone()).run(&rec, &send);
    println!("{}", rec.recv().unwrap());

    Ok(())
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}
