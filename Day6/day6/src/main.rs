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
    //Used to store the last 4 characters
    A: Vec<char>
}

impl Buffer {
    fn shift(&self, newchar: char) -> Self {
        let temp = self.A.to_vec();
        let mut temp2 = Vec::new();
        for letter in temp[1..14].iter() {
            temp2.push(*letter);
        }
        temp2.push(newchar);

        println!("{:?}", temp2);
        Self {
            A: temp2
        }

    }

    fn new(letters: String) -> Self {
        let thevec = letters.chars().collect();
        Self {
            A: thevec
        }
    }

    fn check(&self) -> bool {
        //check if the 14 chars are different
        //fuck fuck fuck fuck fuck fuck
        let mut mythings = Vec::new();
        for thing in self.A.iter() {
            mythings.push(thing)
        }
        //sort and dedup, check length
        mythings.sort();
        mythings.dedup();
        if mythings.len() == 14 {
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
    let mut mybuf = Buffer::new(line[..14].to_string());

    let mut counter: i32 = 14;

    for (i, c) in line.chars().enumerate() {
        if i > 13 {
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
    println!("{counter} ending");
}
