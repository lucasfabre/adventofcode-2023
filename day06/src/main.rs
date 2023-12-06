use clap::Parser;
use std::io::BufRead;

mod waitforit {
    use std::io::BufRead;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct Race {
        time: u64,
        distance: u64,
    }

    fn parse_races(input_stream: Box<dyn BufRead>) -> Vec<Race> {
        // The input looks like this:
        //   Time:      7  15   30
        //   Distance:  9  40  200
        let mut line_itr = input_stream.lines();
        let time_line = line_itr.next().expect("No time line").expect("Failed to read time line");
        let distance_line = line_itr.next().expect("No distance line").expect("Failed to read distance line");

        // Remove the headers of the line
        let time_line = time_line.split_at(7).1;
        let distance_line = distance_line.split_at(10).1;

        let time_values: Vec<u64> = time_line.split_whitespace().map(|s| s.parse::<u64>().expect("Failed to parse time")).collect();
        let distance_values: Vec<u64> = distance_line.split_whitespace().map(|s| s.parse::<u64>().expect("Failed to parse distance")).collect();

        if time_values.len() != distance_values.len() {
            panic!("Time and distance values are not the same length");
        }
        let mut races = Vec::with_capacity(time_values.len());
        for (time, distance) in time_values.iter().zip(distance_values.iter()) {
            races.push(Race {
                time: *time, distance: *distance
            });
        }
        log::debug!("Parsed races: {:?}", races);
        return races;
    }

    fn simulate_race(hold_button_time: u64, record: Race) -> Race {
        // The time actualy represent the speed of the boat, so we can just divide the distance by
        // the time rounding upwards.
        let travel_time = (record.distance + (hold_button_time - 1))/hold_button_time;
        Race {
            time: travel_time + hold_button_time,
            distance: travel_time * hold_button_time,
        }
    }

    impl Race {
        fn compute_nb_of_faster_solutions(&self) -> u64 {
            // Test all the solutions for the range, faster than the Race record time
            let mut nb_of_solutions = 0;
            for hold_button_time in 1..self.time {
                let race = simulate_race(hold_button_time, *self);
                if race.distance >= self.distance && race.time <= self.time && race != *self {
                    log::debug!("Found solution for race {:?}: holding button for {} ms, the race is {:?}", self, hold_button_time, race);
                    nb_of_solutions += 1;
                } else {
                    log::debug!("NOT A solution for race {:?}: holding button for {} ms, the race is {:?}", self, hold_button_time, race);
                }
            }
            log::debug!("There is {:?} solutions for race {:?}", nb_of_solutions, self);
            return nb_of_solutions;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_races() {
            aocstd::init_tests();

            let input_stream: Box<dyn std::io::BufRead> = Box::new(std::io::BufReader::new(
                "Time:      7  15   30\n\
                Distance:   9  40  200"
                .as_bytes()));

            let races = parse_races(input_stream);
            assert!(races.len() == 3);

            let first_race = races[0];
            assert!(first_race.compute_nb_of_faster_solutions() == 4);
        }
   }

    pub fn solve_part1(input_stream: Box<dyn BufRead>) {
        let races = parse_races(input_stream);
        let mut part1_result = 1;
        for race in races {
            let nb_of_solutions = race.compute_nb_of_faster_solutions();
            part1_result *= nb_of_solutions;
        }
        log::info!("Part 1: {}", part1_result);
    }

    pub fn solve_part2(input_stream: Box<dyn BufRead>) {
        // Part2 is the same as part1 but we need to remove the spaces between all the numbers of
        // the input
        let input_content = input_stream.lines().map(|line| line.expect("Failed to read line"))
            .reduce(|line: String, acc: String| { line + "\n" + &acc }).expect("Failed to read input");
        // Use a regex to remove the spaces between the numbers
        log::debug!("Part2 input: {}", input_content);
        let rep_input_content: String = regex::Regex::new(r"(\d)\s+(\d)").unwrap().replace_all(&input_content, "$1$2").to_string();
        log::debug!("Part2 input: {}", rep_input_content);
        // Create a cursor to read the String
        let new_input_stream: Box<dyn BufRead> = Box::new(std::io::Cursor::new(rep_input_content));
        let races = parse_races(new_input_stream);
        let mut part1_result = 1;
        for race in races {
            let nb_of_solutions = race.compute_nb_of_faster_solutions();
            part1_result *= nb_of_solutions;
        }
        log::info!("Part 2: {}", part1_result);
   
    }

}

fn main() {
    let cli = aocstd::Cli::parse();
    aocstd::init_logger(&cli);
    let input_stream: Box<dyn BufRead> = aocstd::get_input_stream(&cli);

    match cli.part {
        aocstd::Part::Part1 => {
            waitforit::solve_part1(input_stream);
        }
        aocstd::Part::Part2 => {
            waitforit::solve_part2(input_stream);
        }
    }
}
