use structopt::StructOpt;
use std::fs;
use std::io;
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::min;
use std::fmt;
use std::error;

fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let file = fs::read_to_string(args.path)?;

    let initial: Vec<&str> = file
        .trim()
        .split('\n')
        .collect();

    let wire1: Wire = initial[0].parse().unwrap();
    let wire2: Wire = initial[1].parse().unwrap();
    let mut min_delay = std::i64::MAX;
    let mut min_distance = std::i64::MAX;
    let mut visited: HashMap<(i64, i64), i64> = HashMap::new();

    wire1.run(|space, delay| { visited.entry(space).or_insert(delay); });
    wire2.run(|space, delay| {
        if let Some(first_delay) = visited.get(&space) {
            min_delay = min(min_delay, delay + first_delay);
            min_distance = min(min_distance, space.0.abs() + space.1.abs());
        }
    });

    println!("min distance {}", min_distance);
    println!("min delay {}", min_delay);
    Ok(())
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Dir {
    UP, DOWN, LEFT, RIGHT
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Segment {
    dir: Dir,
    length: i64,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Wire {
    segments: Vec<Segment>,
}

#[derive(Debug, Clone)]
struct ParseSegmentError;

impl Wire {
    // encapsulate the logic that visits each space of the wire.
    // this could be an iterator instead of using a callback.
    fn run<F>(self, mut func: F) -> ()
        where F: FnMut((i64, i64), i64) -> () {
        let mut cursor = (0, 0);
        let mut delay = 0;
        for segment in self.segments {
            for space in segment.spaces(cursor) {
                delay += 1;
                func(space, delay);
                cursor = space;
            }
        }
    }

}

impl Segment {
    fn spaces(self, (x, y): (i64, i64)) -> Vec<(i64, i64)> {
        let range = 1..self.length+1;
        match self.dir {
            Dir::UP    => range.map(|i| x+i).map(|x| (x, y)).collect(),
            Dir::DOWN  => range.map(|i| x-i).map(|x| (x, y)).collect(),
            Dir::RIGHT => range.map(|i| y+i).map(|y| (x, y)).collect(),
            Dir::LEFT  => range.map(|i| y-i).map(|y| (x, y)).collect(),
        }
    }
}

impl FromStr for Segment {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s.split_at(1);
        let dir = match head.as_ref() {
            "U" => Dir::UP,
            "D" => Dir::DOWN,
            "L" => Dir::LEFT,
            "R" => Dir::RIGHT,
            _ => return Err(ParseSegmentError),
        };
        let length = tail.parse::<i64>()
            .map_err(|_| ParseSegmentError)?;
        Ok(Segment { dir, length })
    }
}

impl FromStr for Wire {
    type Err = ParseSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = s.trim().split(',')
            .map(|i| i.parse())
            .collect::<Result<Vec<Segment>, ParseSegmentError>>()?;
        Ok(Wire { segments })
    }
}

impl fmt::Display for ParseSegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid segment")
    }
}

impl error::Error for ParseSegmentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
