use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn read_line(filepath: &str) -> io::Result<String> {
    //Returns first line of a file
    //Get a buffreader on the file
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file).lines();
    //println!("{:?}", reader.next().unwrap().expect("hhhhhh"));
    let myline = String::from(reader.next().unwrap().expect("File is empty bro"));
    Ok(myline)
}

#[derive(Debug)]
struct Buffer {
    //Used to store the alst 4 characters
    A: char,
    B: char,
    C: char,
    D: char
}

impl Buffer {
    //This is really a getter for a new buffer cause I'm not sure
    //how to modify the existing one
    fn shift(&self, newchar: char) -> Self {
        Self {
            A: newchar,
            B: self.A,
            C: self.B,
            D: self.C
        }
    }

    fn new(a: char, b: char, c: char, d: char) -> Self {
        Self {
            A: a,
            B: b,
            C: c,
            D: d
        }
    }

    fn check(&self) -> bool {
        //check if the 4 chars are different
        let mut mythings = vec![self.A, self.B, self.C, self.D];
        //sort and dedup, check length
        mythings.sort();
        mythings.dedup();
        if mythings.len() == 4 {
            return true;
        } else {
            return false;
        }
    }

}       


fn main() {
    let filepath = "/home/justine/Sandbox/Python/adventofcode2022/Day6/input.txt";
    let line = read_line(filepath).unwrap();
    println!("\tWorking with:\n\n{line}\n\n");
    
    //Counting on the fact that we won't have a match right away
    let mut mybuf = Buffer::new(
        line.chars().nth(3).unwrap(),
        line.chars().nth(2).unwrap(),
        line.chars().nth(1).unwrap(),
        line.chars().nth(0).unwrap()
        );

    let mut counter: i32 = 4;

    for (i, c) in line.chars().enumerate() {
        if i > 3 {
            mybuf = Buffer::shift(&mybuf, c);
            if Buffer::check(&mybuf) {
                dbg!(mybuf);
                counter = counter + 1;
                println!("{counter} characters");
                break;
            } else {
                counter = counter + 1;
            }
        }
    }
}
