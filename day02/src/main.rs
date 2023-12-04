use clap::Parser;
use std::io::BufRead;

mod cube_conundrum {

    use std::collections::HashMap;
    use std::io::BufRead;
    use std::vec::Vec;

    /// A game is represented by each line of the input in the form
    /// ex: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    struct Game {
        id: i32,
        sets: Vec<GameSet>,
    }

    struct GameSet {
        cubes_played: HashMap<CubeColor, NbPlayed>,
    }

    type Inventory = HashMap<CubeColor, NbPlayed>;
    type NbPlayed = i32;
    type CubeColor = String;

    impl Game {
        fn new(line: &str) -> Game {
            log::debug!("Parsing line \"{}\"", line);
            // Parse the line
            // - Step 1 get the game id
            let (game_header, game_body) = {
                let mut parts = line.split(":");
                let game_header = parts
                    .next()
                    .expect("The game does not include a semicolon, is it valid?");
                let game_body = parts
                    .next()
                    .expect("The game does not include a semicolon, is it valid?");
                (game_header, game_body)
            };
            let game_id = {
                // Remove the "Game " prefix
                let game_id = game_header.trim_start_matches("Game ");
                // Parse the game game_id
                game_id
                    .parse::<i32>()
                    .expect("The game id is not a valid integer")
            };

            log::debug!(" - Game id is {}", game_id);

            // - Step 2 get the sets
            let mut sets = Vec::new();
            for set_str in game_body.split(";") {
                sets.push(GameSet::new(set_str));
            }

            Game { id: game_id, sets }
        }

        fn is_game_valid(&self, elf_inventory: &Inventory) -> bool {
            log::debug!(" - Checking if game {} is valid", self.id);
            log::debug!(" - Elf inventory is {:?}", elf_inventory);

            for game_set in &self.sets {
                if !game_set.is_set_valid(elf_inventory) {
                    log::debug!(" - The elf does not have enough cubes to play this game");
                    return false;
                }
            }
            true
        }

        fn get_game_power(&self) -> i64 {
            // build the larger set
            let mut larger_set: Inventory = HashMap::new();
            for game_set in &self.sets {
                for (cube_color, current_set_cube_nb) in &game_set.cubes_played {
                    match larger_set.get(cube_color) {
                        Some(inventory_cube_nb) => {
                            // if the inventory has less cubes than the current set, update the larger set
                            if inventory_cube_nb < current_set_cube_nb {
                                larger_set.insert(cube_color.clone(), *current_set_cube_nb);
                            }
                        }
                        // if the inventory does not have any cube of this color, add it to the larger set
                        None => {
                            larger_set.insert(cube_color.clone(), *current_set_cube_nb);
                        }
                    };
                }
            }
            log::debug!(" - Larger set is {:?}", larger_set);

            // The power of the set is the multiplication of the number of cubes of each cube_color
            let mut power = 1;
            for (_cube_color, nb_played) in &larger_set {
                power *= *nb_played as i64;
            }
            log::debug!(" - Power of the set is {}", power);

            return power;
        }
    }

    impl GameSet {
        fn new(set_str: &str) -> GameSet {
            log::debug!(" - Parsing set \"{}\"", set_str);
            let mut cubes_played = HashMap::new();
            for cube_str in set_str.split(",") {
                let cube_str = cube_str.trim();
                let mut parts = cube_str.split(" ");
                let nb_played = parts
                    .next()
                    .expect("The cube does not include a space, is it valid?");
                let cube_color = parts
                    .next()
                    .expect("The cube does not include a space, is it valid?");
                let nb_played = nb_played
                    .parse::<i32>()
                    .expect("The number of cubes played is not a valid integer");
                cubes_played.insert(cube_color.to_string(), nb_played);
            }
            log::debug!("   - Set is {:?}", cubes_played);
            GameSet { cubes_played }
        }

        fn is_set_valid(&self, elf_inventory: &Inventory) -> bool {
            for (cube_color, nb_played) in &self.cubes_played {
                let nb_owned = elf_inventory.get(cube_color);
                match nb_owned {
                    Some(nb_owned) => {
                        if nb_owned < nb_played {
                            log::debug!(
                                "   - The elf does not have enough {} cubes to play this set",
                                cube_color
                            );
                            return false;
                        }
                    }
                    None => {
                        log::debug!(
                            "   - The elf does not have any {} cubes to play this set",
                            cube_color
                        );
                        return false;
                    }
                }
            }
            true
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_is_game_valid() {
            aocstd::init_tests();

            let elf_inventory: Inventory = HashMap::from([
                (String::from("red"), 12),
                (String::from("green"), 13),
                (String::from("blue"), 14),
            ]);

            let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
            assert!(game1.is_game_valid(&elf_inventory));

            let game3 = Game::new(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            );
            assert!(!game3.is_game_valid(&elf_inventory));
        }

        #[test]
        fn test_get_game_power() {
            aocstd::init_tests();

            let game1 = Game::new("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
            assert_eq!(48, game1.get_game_power());

            let game3 = Game::new(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            );
            assert_eq!(1560, game3.get_game_power());
        }
    }

    pub fn solve_part1(input: Box<dyn BufRead>) -> () {
        let elf_inventory: Inventory = HashMap::from([
            (String::from("red"), 12),
            (String::from("green"), 13),
            (String::from("blue"), 14),
        ]);

        let mut sum_of_valids_game_ids = 0;

        for line in input.lines() {
            let line = line.expect("Could not read line");
            let game = Game::new(&line);
            if game.is_game_valid(&elf_inventory) {
                sum_of_valids_game_ids += game.id;
                log::debug!("Game {} is valid", game.id);
            } else {
                log::debug!("Game {} is invalid", game.id);
            }
        }

        log::info!(
            "The sum of the valid game ids is {}",
            sum_of_valids_game_ids
        );
    }

    pub fn solve_part2(input: Box<dyn BufRead>) -> () {
        let mut sum_of_the_sets_power: i64 = 0;

        for line in input.lines() {
            let line = line.expect("Could not read line");
            let game = Game::new(&line);
            let current_game_power = game.get_game_power();
            sum_of_the_sets_power += current_game_power;
        }

        log::info!("The sum of the sets power is {}", sum_of_the_sets_power);
    }
}

fn main() {
    let cli = aocstd::Cli::parse();
    aocstd::init_logger(&cli);
    let input_stream: Box<dyn BufRead> = aocstd::get_input_stream(&cli);

    match cli.part {
        aocstd::Part::Part1 => {
            cube_conundrum::solve_part1(input_stream);
        }
        aocstd::Part::Part2 => {
            cube_conundrum::solve_part2(input_stream);
        }
    }
}
