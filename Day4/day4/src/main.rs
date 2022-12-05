use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::Range;

fn main(){
    read_lines();
}

fn compute_line(line: String) -> bool {
    let mut s = line.split(",");

    //Splitting our elves into vectors of i32...
    //https://stackoverflow.com/questions/34090639/how-do-i-convert-a-vector-of-strings-to-a-vector-of-integers-in-a-functional-way
    let mut elf1: Result<Vec<i32>, _> = s.next().expect("blah").split("-").map(|x| x.parse()).collect(); 
    let mut elf2: Result<Vec<i32>, _> = s.next().expect("blah").split("-").map(|x| x.parse()).collect();

    let elf1 = elf1.unwrap();
    let elf2 = elf2.unwrap();
    //println!("{:?} - {:?}", elf1, elf2);
    //println!("{:?} - {:?}", range_elf1, range_elf2);

    //elf1 contains elf2
    if (elf1.first() <= elf2.first() && elf1.last() >= elf2.last()) {
        return true;
    //elf2 contains elf1
    } else if (elf2.first() <= elf1.first() && elf2.last() >= elf1.last()) {
        return true;
    } else {
        return false;
    }
}

fn compute_line_p2(line: String) -> bool {
    let mut s = line.split(",");

    //Splitting our elves into vectors of i32...
    //https://stackoverflow.com/questions/34090639/how-do-i-convert-a-vector-of-strings-to-a-vector-of-integers-in-a-functional-way
    let mut elf1: Result<Vec<i32>, _> = s.next().expect("blah").split("-").map(|x| x.parse()).collect(); 
    let mut elf2: Result<Vec<i32>, _> = s.next().expect("blah").split("-").map(|x| x.parse()).collect();

    let elf1 = elf1.unwrap();
    let elf2 = elf2.unwrap();
    //println!("{:?} - {:?}", elf1, elf2);
    //println!("{:?} - {:?}", range_elf1, range_elf2);

    //That means they don't match...
    if (elf2.first() > elf1.last() || elf1.first() > elf2.last()) {
        return false;
    } else {
        return true;
    }
}

//Read the file line by line;
//Sends OK unless one of our ? sends an error
fn read_lines() -> io::Result<()> {
    //Get a buffreader on the file
    let file = File::open("/home/justine/Sandbox/Python/AdventOfCode/Day4/input.txt")?;
    let reader = BufReader::new(file).lines();
    let mut total: i32 = 0;

    for line in reader {
        let current_line = line?;
        if compute_line_p2(current_line) {
            total = total + 1;
        };
    }
    println!("{total}");

    Ok(())
}


