use structopt::StructOpt;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = fs::read_to_string(args.path)?;
    let initial: Vec<i32> = file
        .trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect();

    for noun in 0..99 {
        for verb in 0..99 {
            let mut program = initial.clone();
            program[1] = noun;
            program[2] = verb;
            program = run(program);
            if program[0] == 19690720 {
                println!("{}{}", noun, verb);
            }
        }
    }
    Ok(())
}

fn run(mut program: Vec<i32>) -> Vec<i32> {
    let mut counter = 0;

    loop {
        match program[counter] {
            1 => {
                // add
                let a_index = program[counter+1];
                let b_index = program[counter+2];
                let c_index = program[counter+3];
                program[c_index as usize] = program[a_index as usize] + program[b_index as usize];
                counter+=4;
            }
            2 => {
                // mult
                let a_index = program[counter+1];
                let b_index = program[counter+2];
                let c_index = program[counter+3];
                program[c_index as usize] = program[a_index as usize] * program[b_index as usize];
                counter+=4;
            }
            99 => {
                // halt
                break;
            }
            _ => {
                panic!("encounted unexpected code");
            }
        }
    }
    program
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}
