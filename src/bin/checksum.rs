use structopt::StructOpt;
use std::fs;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let file = fs::read_to_string(args.path)?;
    let mut orbits: HashMap<&str, &str> = HashMap::new();

    for line in file.lines() {
        // "from)to"
        let mut parts = line.splitn(2, ')');
        let from = parts.next().expect("malformed line");
        let to = parts.next().expect("malformed line");
        orbits.insert(to, from);
    }

    let checksum: usize = orbits.keys()
        .map(|key| path(key, &orbits).len())
        .sum();
    println!("checksum: {}", checksum);

    let san = orbits.get("SAN").expect("SAN is not on the map");
    let index = path(san, &orbits);
    let mut depth = 0;
    let mut iter = "YOU";
    while let Some(x) = orbits.get(iter) {
        iter = x;
        if let Some(d)  = index.get(iter) {
            depth += d;
            break;
        }
        depth+=1;
    }
    println!("depth: {}", depth);

    Ok(())
}

fn path<'a>(key: &'a str, orbits: &'a HashMap<&str, &str>) -> HashMap<&'a str, i32> {
    let mut map: HashMap<&'a str, i32> = HashMap::new();
    let mut iter = key;
    let mut l = 0;
    while let Some(x) = orbits.get(iter) {
        map.insert(iter, l);
        iter = x;
        l+=1;
    }
    map
}

#[derive(StructOpt)]
struct Cli {
    path: String,
}
