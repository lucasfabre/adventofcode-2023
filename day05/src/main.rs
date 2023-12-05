use clap::Parser;
use std::io::BufRead;

mod giveaseedafertilizer {
    use regex::Regex;
    use std::collections::BTreeSet as Set;
    use std::io::BufRead;

    // The almanac contains a list of transofrmations to apply to the seeds
    // they are represented by maps of the form:
    //   seed-to-soil map:
    //   50 98 2
    //   52 50 48
    // where each line is:
    //   <destination category> <source start range> <source range>
    // Every transformation is applied the same way and the almanac seems to be in order, so we are
    // using that to build a generic vector of transformations to apply
    #[derive(Debug)]
    struct Almanac {
        seeds: Set<SeedRange>,
        transformation_maps: Vec<TransformationMap>,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
    struct SeedRange {
        start: u64,
        length: u64,
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum SeedParsingMode {
        OneSeed,
        SeedRange,
    }

    #[derive(Debug)]
    struct TransformationMap {
        transformations: Vec<Transformation>,
    }

    #[derive(Debug)]
    struct Transformation {
        destination_category: u64,
        source_start_range: u64,
        source_range: u64,
    }

    impl Almanac {
        fn from_input_stream(
            input_stream: Box<dyn BufRead>,
            seed_parsing_mode: SeedParsingMode,
        ) -> Self {
            // Read the input stream line by line with an iterator
            let mut line_itr = input_stream.lines();

            // The first line is the list of seeds in the form:
            // seeds: 79 14 55 13
            let seeds_line: String = line_itr
                .next()
                .expect("No seeds line found")
                .expect("Cannot read seeds line");
            // Do a quick check with a regex to make sure the line is well formed and avoid unecessary debugging
            if !Regex::new(r"^seeds: \d+( \d+)*$")
                .unwrap()
                .is_match(&seeds_line)
            {
                panic!("Invalid seeds line: {}", seeds_line);
            }

            let nb_from_seed_line = seeds_line
                .split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().expect("Cannot parse seed"))
                .collect::<Vec<u64>>();
            let seeds = if seed_parsing_mode == SeedParsingMode::SeedRange {
                // In seed range mode the first number represent the start of the range and the second the length
                let mut last_seed = 0;
                let mut result_seeds: Set<SeedRange> = Set::new();
                for (index, current_nb) in nb_from_seed_line.iter().enumerate() {
                    log::debug!("index: {}, current_nb: {}", index, current_nb);
                    if index % 2 == 0 {
                        last_seed = *current_nb;
                    } else {
                        // push the range of seeds
                        result_seeds.insert(SeedRange {
                            start: last_seed,
                            length: *current_nb,
                        });
                    }
                }
                result_seeds
            } else {
                // In one seed mode each number represent a seed with a 1 length
                nb_from_seed_line
                    .iter()
                    .map(|s| SeedRange {
                        start: *s,
                        length: 1,
                    })
                    .collect::<Set<SeedRange>>()
            };
            log::debug!("Found seeds: {:?}", seeds);

            // Read the next line and assert that it is empty
            let empty_line: String = line_itr
                .next()
                .expect("No empty line found")
                .expect("Cannot read empty line");
            if !empty_line.is_empty() {
                panic!("Expected empty line, found: {}", empty_line);
            }

            // Read each transformation map
            let mut transformation_maps = Vec::new();
            while let Some(transformation_map) = TransformationMap::from(&mut line_itr) {
                log::debug!("Found transformation map: {:?}", transformation_map);
                transformation_maps.push(transformation_map);
            }

            return Almanac {
                seeds,
                transformation_maps,
            };
        }

        fn apply_transformations_and_keep_lower_result(&self) -> u64 {
            let mut lower_result: Option<u64> = None;
            for seedrange in self.seeds.iter() {
                for seed in seedrange.start..seedrange.start + seedrange.length {
                    let mut transformation_result = seed;
                    for transformation_map in &self.transformation_maps {
                        transformation_result =
                            transformation_map.apply_transformation(transformation_result);
                    }
                    log::debug!("Seed: {}, result: {}", seed, transformation_result);
                    if lower_result.is_none() || transformation_result < lower_result.unwrap() {
                        lower_result = Some(transformation_result);
                    }
                }
            }
            log::debug!("Lower result: {:?}", lower_result);
            return lower_result.unwrap();
        }
    }

    impl TransformationMap {
        fn from(line_itr: &mut dyn Iterator<Item = std::io::Result<String>>) -> Option<Self> {
            // Read the transformation map header (ex: "seed-to-soil map:")
            let header_line = line_itr.next();
            // Check the different error cases
            let header_line = match header_line {
                None => return None,
                Some(Err(e)) => panic!("Cannot read transformation map header: {}", e),
                Some(Ok(line)) => line,
            };
            if !Regex::new(r"^\w+-to-\w+ map:$")
                .unwrap()
                .is_match(&header_line)
            {
                return None;
            }
            log::debug!("Found transformation map header: {}", header_line);
            let mut transformations = Vec::new();
            // Read the next lines until we find an empty line
            while let Some(line) = line_itr.next() {
                let line = line.expect("Cannot read transformation map line");
                if line.is_empty() {
                    break;
                }
                transformations.push(Transformation::from(&line));
            }

            return Some(TransformationMap { transformations });
        }

        fn apply_transformation(&self, initial_value: u64) -> u64 {
            for transformation in &self.transformations {
                let transformation_result: Option<u64> =
                    transformation.apply_transformation(initial_value);
                if let Some(transformation_result) = transformation_result {
                    return transformation_result;
                }
            }
            return initial_value;
        }
    }

    impl Transformation {
        fn from(line: &str) -> Self {
            // The transformation is a line of the form:
            // 50 98 2
            // where each number is:
            // <destination category> <source start range> <source range>
            let mut numbers = line
                .split_ascii_whitespace()
                .map(|s| {
                    s.parse::<u64>()
                        .expect("Cannot parse transformation number")
                })
                .collect::<Vec<u64>>();
            if numbers.len() != 3 {
                panic!("Invalid transformation line: {}", line);
            }
            let destination_category = numbers.remove(0);
            let source_start_range = numbers.remove(0);
            let source_range = numbers.remove(0);
            return Transformation {
                destination_category,
                source_start_range,
                source_range,
            };
        }

        fn apply_transformation(&self, initial_value: u64) -> Option<u64> {
            if initial_value >= self.source_start_range
                && initial_value < self.source_start_range + self.source_range
            {
                let delta = initial_value - self.source_start_range;
                return Some(self.destination_category + delta);
            } else {
                return None;
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_transformation_one_seed() {
            aocstd::init_tests();

            let input_stream: Box<dyn std::io::BufRead> = Box::new(std::io::BufReader::new(
                "seeds: 79 14 55 13\n\
                \n\
                seed-to-soil map:
                50 98 2
                52 50 48\n\
                \n\
                soil-to-fertilizer map:\n\
                0 15 37\n\
                37 52 2\n\
                39 0 15"
                    .as_bytes(),
            ));

            let almanac = Almanac::from_input_stream(input_stream, SeedParsingMode::OneSeed);
            let seed_transformation_result = almanac.apply_transformations_and_keep_lower_result();
            assert_eq!(seed_transformation_result, 52)
        }
    }

    pub fn solve_part1(input_stream: Box<dyn BufRead>) {
        let almanac = Almanac::from_input_stream(input_stream, SeedParsingMode::OneSeed);
        let lowest_result = almanac.apply_transformations_and_keep_lower_result();
        log::info!("Part1: {:?}", lowest_result);
    }

    pub fn solve_part2(input_stream: Box<dyn BufRead>) {
        let almanac = Almanac::from_input_stream(input_stream, SeedParsingMode::SeedRange);
        let lowest_result = almanac.apply_transformations_and_keep_lower_result();
        log::info!("Part2: {:?}", lowest_result);
    }
}

fn main() {
    let cli = aocstd::Cli::parse();
    aocstd::init_logger(&cli);
    let input_stream: Box<dyn BufRead> = aocstd::get_input_stream(&cli);

    match cli.part {
        aocstd::Part::Part1 => {
            giveaseedafertilizer::solve_part1(input_stream);
        }
        aocstd::Part::Part2 => {
            giveaseedafertilizer::solve_part2(input_stream);
        }
    }
}
