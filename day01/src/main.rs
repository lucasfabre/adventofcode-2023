use std::collections::BTreeMap;

fn main() {
    // Create a treemap with the text representation of the digits as keys and the value of the digit as value,
    // required for the second part
    let digits : BTreeMap<&'static str, u32> = BTreeMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    // create the resulting count variable
    let mut count : u64 = 0;

    // Iterate over each line of stdin
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        // find the first and the last Digit of the line
        let mut first : Option<u64> = None;
        let mut last : Option<u64> = None;

        for (index, character) in line.chars().enumerate() {
            let mut current_digit: Option<u64> = None;

            if character.is_digit(10) {
                current_digit = Some(character.to_digit(10).unwrap() as u64);
            } else {
                // if it is not a Digit we need to check if it is a digit from the enum
                for (digit_name, digit_value) in digits.iter() {
                    // if the rest of the line is shorter than the name of the Digit, we can skip the rest of the line
                    if index + digit_name.len() <= line.len() {
                        // Create a slice of tjhe line from the current index to the end of the matching Digit
                        let slice = &line[index..index + digit_name.len()];
                        if slice == *digit_name {
                            // if the slice is equal to the name of the Digit, we can add the value of the digit to the count
                            current_digit = Some(*digit_value as u64);
                        }
                    }
                }
            }

            // We have found a Digit
            if current_digit.is_some() {
                if first.is_none() {
                    first = current_digit;
                }
                last = current_digit;
            }
        }

        // Create the line number by associating the two Digits
        let line_number = match (first, last) {
            (Some(f), Some(l)) => f * 10 + l,
            _ => 0,
        };

        count += line_number;
    }

    println!("the calibration values are {}", count);
}
