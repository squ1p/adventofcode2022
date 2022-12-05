fn main() {
    println!("Hello, world!");
}

//Read the file line by line;
//Sends OK unless one of our ? sends an error
fn read_lines() -> io::Result<()> {
    //Get a buffreader on the file
    let file = File::open("/home/justine/Sandbox/Python/AdventOfCode/Day5/input.txt")?;
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