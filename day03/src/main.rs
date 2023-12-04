use clap::Parser;
use std::io::BufRead;

mod gear_ratios {

    use std::collections::HashMap;
    use std::io::BufRead;

    #[derive(Debug, Clone, Copy)]
    enum SchematicPart {
        Nothing,
        Symbol(char),
        PartialPartId(char),
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct Position {
        x: u32,
        y: u32,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct PartId {
        id: u32,
        position: Position,
        length: u32,
    }

    /// Used to return more informations about the symbols when checking if a part id is next to a symbol
    /// This is used to determine if a symbol is a gear
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    struct SymbolInformations {
        symbol: char,
        position: Position,
    }

    struct Schematic {
        map: Vec<Vec<SchematicPart>>,
    }

    impl Schematic {
        fn from_input_stream(input_stream: Box<dyn BufRead>) -> Self {
            let map = Schematic::build_map(input_stream);
            Schematic { map }
        }

        fn build_map(input_stream: Box<dyn BufRead>) -> Vec<Vec<SchematicPart>> {
            let mut map = Vec::new();
            for line in input_stream.lines() {
                let mut row = Vec::new();
                for c in line.unwrap().chars() {
                    // Determine the schematic_part of the current character
                    let schematic_part = {
                        if c == '.' {
                            SchematicPart::Nothing
                        } else if c.is_digit(10) {
                            SchematicPart::PartialPartId(c)
                        } else {
                            SchematicPart::Symbol(c)
                        }
                    };
                    // Add the schematic_part to the row
                    row.push(schematic_part);
                }
                // Add the row to the map
                map.push(row);
            }
            return map;
        }

        fn identify_part_ids(&self) -> Vec<PartId> {
            let mut part_ids = Vec::new();

            for (y, row) in self.map.iter().enumerate() {
                let mut current_part_id: Option<PartId> = None;

                for (x, part) in row.iter().enumerate() {
                    if let SchematicPart::PartialPartId(c) = part {
                        match current_part_id {
                            None => {
                                current_part_id = Some(PartId {
                                    id: c.to_digit(10).expect("Invalid part id"),
                                    position: Position {
                                        x: x as u32,
                                        y: y as u32,
                                    },
                                    length: 1,
                                });
                            }
                            Some(id) => {
                                current_part_id = Some(PartId {
                                    id: id.id * 10 + c.to_digit(10).expect("Invalid part id"),
                                    position: id.position,
                                    length: id.length + 1,
                                });
                            }
                        }
                    } else {
                        if let Some(id) = current_part_id {
                            part_ids.push(id);
                            current_part_id = None;
                        }
                    }
                }
                if let Some(id) = current_part_id {
                    part_ids.push(id);
                }
            }
            return part_ids;
        }

        fn print(&self, log_level: log::Level) {
            for row in &self.map {
                let mut row_str: String = String::with_capacity(row.len());
                for part in row {
                    match part {
                        SchematicPart::Nothing => row_str.push('.'),
                        SchematicPart::Symbol(c) => row_str.push(*c),
                        SchematicPart::PartialPartId(c) => row_str.push(*c),
                    }
                }
                log::log!(log_level, "{}", row_str);
            }
        }
    }

    impl PartId {
        fn scan_adjacent_symbols(&self, schematic: &Schematic) -> Vec<SymbolInformations> {
            let position = self.position.clone();
            let mut symbols = Vec::new();

            for x in -1..(self.length as i32 + 1) {
                for y in -1..2 {
                    let current_x_scanned = position.x as i32 + x;
                    let current_y_scanned = position.y as i32 + y;

                    // Check that we are in bounds
                    if current_y_scanned >= 0
                        && current_y_scanned < schematic.map.len() as i32
                        && current_x_scanned >= 0
                        && current_x_scanned
                            < schematic.map[current_y_scanned as usize].len() as i32
                    {
                        let part =
                            schematic.map[current_y_scanned as usize][current_x_scanned as usize];
                        match part {
                            SchematicPart::Nothing => {}
                            SchematicPart::Symbol(c) => {
                                symbols.push(SymbolInformations {
                                    symbol: c,
                                    position: Position {
                                        x: current_x_scanned as u32,
                                        y: current_y_scanned as u32,
                                    },
                                });
                            }
                            SchematicPart::PartialPartId(_) => {}
                        }
                    }
                }
            }
            return symbols;
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn identify_part_ids_and_scan_adjacent_symbols() {
            aocstd::init_tests();

            let input_stream: Box<dyn std::io::BufRead> = Box::new(std::io::BufReader::new(
                "467..114..\n\
                 ...*......\n\
                 ..35..633.\n\
                 ......#..."
                    .as_bytes(),
            ));

            let schematic = Schematic::from_input_stream(input_stream);
            let part_ids = schematic.identify_part_ids();
            assert_eq!(
                part_ids,
                vec![
                    PartId {
                        id: 467,
                        position: Position { x: 0, y: 0 },
                        length: 3
                    },
                    PartId {
                        id: 114,
                        position: Position { x: 5, y: 0 },
                        length: 3
                    },
                    PartId {
                        id: 35,
                        position: Position { x: 2, y: 2 },
                        length: 2
                    },
                    PartId {
                        id: 633,
                        position: Position { x: 6, y: 2 },
                        length: 3
                    }
                ]
            );
            let adjacent_symbols = part_ids[0].scan_adjacent_symbols(&schematic);
            assert_eq!(
                adjacent_symbols,
                vec![SymbolInformations {
                    symbol: '*',
                    position: Position { x: 3, y: 1 }
                },]
            );
            assert_eq!(part_ids[1].scan_adjacent_symbols(&schematic), vec![]);
        }
    }

    pub fn solve_part1(input_stream: Box<dyn BufRead>) {
        let schematic = Schematic::from_input_stream(input_stream);
        log::debug!("Schematic:");
        schematic.print(log::Level::Debug);
        let part_ids = schematic.identify_part_ids();
        log::debug!("Part ids: {:?}", part_ids);

        // check witch part ids are next to a symbol and build the sum of the part_ids
        let mut sum = 0;
        for part_id in part_ids {
            if part_id.scan_adjacent_symbols(&schematic).len() > 0 {
                log::debug!("Part id {} is next to a symbol", part_id.id);
                sum += part_id.id;
            }
        }
        log::info!("Sum of part ids: {}", sum);
    }

    pub fn solve_part2(input_stream: Box<dyn BufRead>) {
        let schematic = Schematic::from_input_stream(input_stream);
        log::debug!("Schematic:");
        schematic.print(log::Level::Debug);
        let part_ids = schematic.identify_part_ids();
        log::debug!("Part ids: {:?}", part_ids);

        let mut potential_gears: HashMap<SymbolInformations, Vec<PartId>> = HashMap::new();

        // find all the adjacent symbols for each part id in order to find the gears
        for part_id in part_ids {
            let adjacent_symbols = part_id.scan_adjacent_symbols(&schematic);
            for symbol in adjacent_symbols {
                // The gear always has a '*' symbol
                if symbol.symbol == '*' {
                    if let Some(part_ids) = potential_gears.get_mut(&symbol) {
                        part_ids.push(part_id.clone());
                    } else {
                        potential_gears.insert(symbol, vec![part_id.clone()]);
                    }
                }
            }
        }

        let gears = potential_gears
            .iter()
            .filter(|(_, part_ids)| part_ids.len() == 2)
            .collect::<Vec<(&SymbolInformations, &Vec<PartId>)>>();
        let gear_ratios = gears
            .iter()
            .map(|(_symbol, part_ids)| part_ids[0].id as u64 * part_ids[1].id as u64)
            .reduce(|a, b| a + b)
            .unwrap();

        log::info!("Gear ratios: {}", gear_ratios);
    }
}

fn main() {
    let cli = aocstd::Cli::parse();
    aocstd::init_logger(&cli);
    let input_stream: Box<dyn BufRead> = aocstd::get_input_stream(&cli);

    match cli.part {
        aocstd::Part::Part1 => {
            gear_ratios::solve_part1(input_stream);
        }
        aocstd::Part::Part2 => {
            gear_ratios::solve_part2(input_stream);
        }
    }
}
