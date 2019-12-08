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

    println!("part 1");
    let program = Program { memory: initial.clone(), counter: 0 };
    program.run(1);

    println!("part 2");
    let program = Program { memory: initial.clone(), counter: 0 };
    program.run(5);
    Ok(())
}

struct Program {
    counter: usize,
    memory: Vec<i32>,
}

impl Program {
    fn read_opcode(&mut self) -> (i32, i32) {
        let opcode = self.memory[self.counter] % 100;
        let paramode = self.memory[self.counter] / 100;
        self.counter+=1;
        (opcode, paramode)
    }

    fn read_params(&mut self, l: usize, mut paramode: i32) -> Vec<usize> {
        let mut vec = Vec::with_capacity(l);
        for _ in 0..l {
            let mode = paramode % 10;
            paramode = paramode / 10;
            let idx =  match mode {
                1 => self.counter,
                0 => self.memory[self.counter] as usize,
                _ => panic!("unknown paramode {}", paramode),
            };
            vec.push(idx);
            self.counter+=1;
        }
        vec
    }

    fn run(mut self, input: i32) {
        loop {
            let (opcode, paramode) = self.read_opcode();
            match opcode {
                1 => {
                    // add 2=0+1
                    let params = self.read_params(3, paramode);
                    self.memory[params[2]] = self.memory[params[0]] + self.memory[params[1]];
                },
                2 => {
                    // mul 2=0*1
                    let params = self.read_params(3, paramode);
                    self.memory[params[2]] = self.memory[params[0]] * self.memory[params[1]];
                },
                3 => {
                    // input 0=input
                    let params = self.read_params(1, paramode);
                    self.memory[params[0] as usize] = input;
                },
                4 => {
                    // output output=0
                    let params = self.read_params(1, paramode);
                    println!("{}", self.memory[params[0]]);
                },
                5 => {
                    // jump if false 0
                    let params = self.read_params(2, paramode);
                    if self.memory[params[0]] != 0 {
                        self.counter = self.memory[params[1]] as usize;
                    }
                },
                6 => {
                    // jump if true 0
                    let params = self.read_params(2, paramode);
                    if self.memory[params[0]] == 0 {
                        self.counter = self.memory[params[1]] as usize;
                    }
                },
                7 => {
                    // less than 2 = 0<1 ? 1 : 0
                    let params = self.read_params(3, paramode);
                    self.memory[params[2]] = if self.memory[params[0]] < self.memory[params[1]] {
                        1
                    } else {
                        0
                    }
                },
                8 => {
                    // equals 2 = 0==1 ? 1  : 0
                    let params = self.read_params(3, paramode);
                    self.memory[params[2]] = if self.memory[params[0]] == self.memory[params[1]] {
                        1
                    } else {
                        0
                    }
                },
                99 => {
                    break;
                },
                _ => panic!("unknown opcode {}", opcode),
            };
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}
