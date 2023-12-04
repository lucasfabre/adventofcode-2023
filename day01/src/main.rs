use clap::Parser;
use std::io::BufRead;

mod trebuchet {

    use phf::phf_map;
    use std::io::BufRead;

    type CalibrationValue = u8;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum IdentificationMode {
        Digit,
        DigitAndName,
    }

    /// Digits and their associated values
    /// We are using phf crate to create a static Map
    static DIGITS: phf::Map<&'static str, u8> = phf_map! {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
    };

    fn identify_calibration_value_single_line(
        line: &str,
        identification_mode: IdentificationMode,
    ) -> CalibrationValue {
        let mut first: Option<u8> = None;
        let mut last: Option<u8> = None;

        for (index, character) in line.chars().enumerate() {
            let mut current_digit: Option<u8> = None;

            if character.is_digit(10) {
                current_digit = Some(character.to_digit(10).expect("Invalid digit") as u8);
            } else if identification_mode == IdentificationMode::DigitAndName {
                // if it is not a Digit we need to check if it is a digit from the enum
                for (digit_name, digit_value) in DIGITS.entries() {
                    // if the rest of the line is shorter than the name of the Digit, we can skip the rest of the line
                    if index + digit_name.len() <= line.len() {
                        // Create a slice of tjhe line from the current index to the end of the matching Digit
                        let slice = &line[index..index + digit_name.len()];
                        if slice == *digit_name {
                            // if the slice is equal to the name of the Digit, we can add the value of the digit to the count
                            current_digit = Some(*digit_value as u8);
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

        // find the first and the last Digit of the line
        // Create the line number by associating the two Digits
        let calibration_value = match (first, last) {
            (Some(f), Some(l)) => f * 10 + l,
            _ => 0,
        };

        log::debug!("line=[{}] calibration_value=[{}]", line, calibration_value);
        return calibration_value;
    }

    fn identify_calibration_values(
        input_stream: Box<dyn BufRead>,
        identification_mode: IdentificationMode,
    ) -> Vec<CalibrationValue> {
        let mut calibration_values = Vec::new();

        for line in input_stream.lines() {
            let line = line.expect("Cannot read line");
            let calibration_value =
                identify_calibration_value_single_line(&line, identification_mode);
            calibration_values.push(calibration_value);
        }

        return calibration_values;
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_digits_only() {
            aocstd::init_tests();

            let input_stream = Box::new(std::io::BufReader::new(
                "1abc2\n\
                 pqr3stu8vwx\n\
                 a1b2c3d4e5f\n\
                 treb7uchet"
                    .as_bytes(),
            ));
            let calibration_values =
                super::identify_calibration_values(input_stream, super::IdentificationMode::Digit);
            assert_eq!(calibration_values, vec![12, 38, 15, 77]);
        }

        #[test]
        fn test_digits_and_names() {
            aocstd::init_tests();

            let input_stream = Box::new(std::io::BufReader::new(
                "two1nine\n\
                 eightwothree\n\
                 abcone2threexyz\n\
                 xtwone3four\n\
                 4nineeightseven2\n\
                 zoneight234\n\
                 7pqrstsixteen"
                    .as_bytes(),
            ));

            let calibration_values = super::identify_calibration_values(
                input_stream,
                super::IdentificationMode::DigitAndName,
            );
            assert_eq!(calibration_values, vec![29, 83, 13, 24, 42, 14, 76]);
        }
    }

    pub fn solve_part1(input_stream: Box<dyn BufRead>) {
        let calibration_values =
            identify_calibration_values(input_stream, IdentificationMode::Digit);
        let sum: u32 = calibration_values.iter().map(|x| *x as u32).sum();
        log::info!("Part 1: {}", sum);
    }

    pub fn solve_part2(input_stream: Box<dyn BufRead>) {
        let calibration_values =
            identify_calibration_values(input_stream, IdentificationMode::DigitAndName);
        let sum: u32 = calibration_values.iter().map(|x| *x as u32).sum();
        log::info!("Part 2: {}", sum);
    }
}

fn main() {
    let cli = aocstd::Cli::parse();
    aocstd::init_logger(&cli);
    let input_stream: Box<dyn BufRead> = aocstd::get_input_stream(&cli);

    match cli.part {
        aocstd::Part::Part1 => {
            trebuchet::solve_part1(input_stream);
        }
        aocstd::Part::Part2 => {
            trebuchet::solve_part2(input_stream);
        }
    }
}
