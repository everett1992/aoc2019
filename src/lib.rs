use futures::channel::mpsc::{channel,Sender,Receiver};
use futures::prelude::*;

pub struct IntCode {
    counter: usize,
    memory: Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
}

impl IntCode {
    pub fn new(memory: Vec<i32>) -> (IntCode, Sender<i32>, Receiver<i32>) {
        let (tx, input_rx) = channel(20);
        let (output_tx, rx) = channel(20);
        let intcode= IntCode {
            memory,
            counter: 0,
            input: input_rx,
            output: output_tx,
        };
        (intcode, tx, rx)
    }

    pub async fn run(mut self) -> () {
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
                    self.memory[params[0] as usize] = self.input.next().await.unwrap();
                },
                4 => {
                    // output output=0
                    let params = self.read_params(1, paramode);
                    self.output.send(self.memory[params[0]]).await.unwrap();
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

}
