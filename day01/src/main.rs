fn main() {
    // create the resulting count variable
    let mut count : u64 = 0;

    // While stdin is not empty
    // read the line
    for line in std::io::stdin().lines() {
        // find the first and the last digit of the line
        let mut first : Option<u64> = None;
        let mut last : Option<u64> = None;

        for c in line.unwrap().chars() {
            if c.is_digit(10) {
                if first == None {
                    first = Some(c.to_digit(10).unwrap() as u64);
                }
                last = Some(c.to_digit(10).unwrap() as u64);
            }
        }

        // Create the line number by associating the two digits
        let line_number = match (first, last) {
            (Some(f), Some(l)) => f * 10 + l,
            _ => 0,
        };

        count += line_number;
    }

    println!("the calibration values are {}", count);
}
