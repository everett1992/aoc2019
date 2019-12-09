use structopt::StructOpt;
use std::fs;
use std::io;
use aoc::IntCode;
use futures::channel::mpsc::{Sender,Receiver};
use futures::prelude::*;
use futures::join;
use futures::executor::block_on;


fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = fs::read_to_string(args.path)?;
    let initial: Vec<i32> = file
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    let mut max = std::i32::MIN;
    for code in permutations(vec![5,6,7,8,9]) {
        let thrust = block_on(test(initial.clone(), code));
        max = std::cmp::max(max, thrust);
    }
    println!("{}", max);
    Ok(())
}

async fn test(initial: Vec<i32>, code: Vec<i32>) -> i32 {
    let (amp_a, mut tx_a, rx_a) = IntCode::new(initial.clone());
    let (amp_b, mut tx_b, rx_b) = IntCode::new(initial.clone());
    let (amp_c, mut tx_c, rx_c) = IntCode::new(initial.clone());
    let (amp_d, mut tx_d, rx_d) = IntCode::new(initial.clone());
    let (amp_e, mut tx_e, rx_e) = IntCode::new(initial.clone());

    // initialization code.
    tx_a.send(code[0]).await.expect("could not initialize amps");
    tx_b.send(code[1]).await.expect("could not initialize amps");
    tx_c.send(code[2]).await.expect("could not initialize amps");
    tx_d.send(code[3]).await.expect("could not initialize amps");
    tx_e.send(code[4]).await.expect("could not initialize amps");

    // start the feedback cycle.
    tx_a.send(0).await.expect("could not send first data");

    let (.., output) = join!(
        amp_a.run(),
        amp_b.run(),
        amp_c.run(),
        amp_d.run(),
        amp_e.run(),
        last(rx_a, tx_b),
        last(rx_b, tx_c),
        last(rx_c, tx_d),
        last(rx_d, tx_e),
        last(rx_e, tx_a),
    );

    output.unwrap()
}

async fn last(mut rx: Receiver<i32>, mut tx: Sender<i32>) -> Option<i32>{
    let mut last = None;
    while let Some(data) = rx.next().await {
        last = Some(data);
        if let Err(_) = tx.send(data).await {
            break;
        }
    }
    last
}

fn permutations(set: Vec<i32>) -> Vec<Vec<i32>> {
    if set.len() == 1 {
        return vec![set.clone()];
    }

    let mut results = Vec::new();
    for (idx, x) in set.iter().enumerate() {
        let mut remaining = set.clone();
        remaining.remove(idx);
        for mut suffix in permutations(remaining) {
            suffix.push(*x);
            results.push(suffix);
        }
    }
    results
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}

