use clap::Parser;
use std::io::BufRead;

mod scratchcards {
    use std::io::BufRead;

    /// A card contains a set of winning numbers and a set of numbers represented by:
    /// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    /// where the first 5 numbers are the winning numbers and the last 8 numbers are the numbers of the card
    struct Card {
        id: u32,
        winning_numbers: Vec<u32>,
        numbers: Vec<u32>,
    }

    struct CardSet {
        cards: Vec<Card>,
    }

    impl Card {
        fn from_line(line: &str) -> Self {
            log::debug!("Parsing line: {}", line);

            // Compute some helper indexes to split the line
            let end_header_index = line.find(':').expect("No ':' found in line");
            let end_winning_numbers_index = line.find('|').expect("No '|' found in line");

            // Split the line into the different parts:
            // the header part contians the card id:                  Card 1
            // the winning numbers part contains the winning numbers: 41 48 83 86 17
            // the numbers part contains the numbers of the card:     83 86  6 31 17  9 48 53
            let header_part_of_the_line: &str = line[0..end_header_index].trim();
            let winning_numbers_part_of_the_line: &str =
                line[end_header_index + 1..end_winning_numbers_index].trim();
            let numbers_part_of_the_line: &str = line[end_winning_numbers_index + 1..].trim();
            log::debug!(
                "found parts of the line: header=[{}], winning_numbers=[{}], numbers=[{}]",
                header_part_of_the_line,
                winning_numbers_part_of_the_line,
                numbers_part_of_the_line
            );

            // Parse every parts into the corresponding data structure
            let id = header_part_of_the_line[5..header_part_of_the_line.len()]
                .trim()
                .parse::<u32>()
                .expect("Cannot parse card id");
            let winning_numbers = winning_numbers_part_of_the_line
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| {
                    n.trim()
                        .parse::<u32>()
                        .expect("Cannot parse winning number")
                })
                .collect::<Vec<u32>>();
            let numbers = numbers_part_of_the_line
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.trim().parse::<u32>().expect("Cannot parse number"))
                .collect::<Vec<u32>>();

            // Return the Card
            Card {
                id,
                winning_numbers,
                numbers,
            }
        }

        fn compute_nb_of_matching_numbers(&self) -> u32 {
            let mut nb_of_matching_numbers = 0;
            for number in &self.numbers {
                if self.winning_numbers.contains(number) {
                    nb_of_matching_numbers += 1;
                    log::debug!("Found winning number {} for card {}", number, self.id);
                }
            }
            return nb_of_matching_numbers;
        }
    }

    impl CardSet {
        fn from_input_stream(input_stream: Box<dyn BufRead>) -> Self {
            let mut card_set = Vec::new();
            for line in input_stream.lines() {
                let card = Card::from_line(line.expect("Cannot read line").as_str());
                card_set.push(card);
            }
            log::debug!("Found {} cards in CardSet", card_set.len());
            CardSet { cards: card_set }
        }

        /// Returns the total number of points won by the card set
        /// The ruleset 1 concerns the first part of the exercise when the individual cards win
        /// points
        fn nb_of_points_won_with_ruleset1(&self) -> u32 {
            let mut nb_of_points_won = 0;
            for card in self.cards.iter() {
                let nb_of_matching_numbers = card.compute_nb_of_matching_numbers();
                let nb_of_points_won_by_card = if nb_of_matching_numbers > 0 {
                    2u32.pow(nb_of_matching_numbers - 1)
                } else {
                    0u32
                };
                nb_of_points_won += nb_of_points_won_by_card;
            }
            log::debug!("Found {} points won in CardSet", nb_of_points_won);
            return nb_of_points_won;
        }

        /// For the ruleset2 we need to compute the nb of card won.
        /// each matching nb of one card give a subsequent card to the player in the following fashion:
        /// Card 1: has 2 matching numbers, so the player wins one copy of the next 2 card (Card 2 and Card 3)
        /// Card 2: has 1 matching number, so the player wins one copy of the next card (Card 3)
        /// Card 3: has 1 matching number, because the player as 2 copies of Card 3, he wins two copy of the next card (Card 4)
        /// Card 4: has 0 matching number, so game ends
        fn nb_of_cards_won_with_ruleset2(&self) -> u32 {
            // We starts with one copy of each card in the input
            let mut nb_of_copy_of_cards: Vec<u32> = vec![1; self.cards.len()];
            for (current_card_index, current_card) in self.cards.iter().enumerate() {
                let nb_of_copy_of_current_card = nb_of_copy_of_cards[current_card_index];
                log::debug!(
                    "Processing card {} with {} copies",
                    current_card_index,
                    nb_of_copy_of_current_card
                );
                let nb_of_matching_numbers = current_card.compute_nb_of_matching_numbers();
                let mut cards_indexes_won = Vec::new();
                // Compute the indexes of the cards won by the current card
                for i in 0..nb_of_matching_numbers {
                    let card_id_won = current_card_index + i as usize + 1;
                    // Cards will never make you copy a card past the end of the table
                    if card_id_won < self.cards.len() {
                        cards_indexes_won.push(card_id_won);
                    }
                }
                log::debug!(
                    "Found {} cards won by card {}, indexes (NOT IDs!) are: {:?}",
                    nb_of_matching_numbers,
                    current_card.id,
                    cards_indexes_won
                );
                // For each card won, we add the number of copy of the current card to the number of copy of the card won
                for card_index_won in cards_indexes_won {
                    nb_of_copy_of_cards[card_index_won] += nb_of_copy_of_current_card;
                }
            }
            // Compute the total number of cards won
            let nb_of_cards_won: u32 = nb_of_copy_of_cards.iter().sum();
            log::debug!(
                "Found {} cards won in CardSet, nb_of_copy_of_cards={:?}",
                nb_of_cards_won,
                nb_of_copy_of_cards
            );
            return nb_of_cards_won;
        }
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn test_card() {
            aocstd::init_tests();

            let card = super::Card::from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
            assert_eq!(card.id, 1);
            assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
            assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);

            let nb_of_matching_numbers = card.compute_nb_of_matching_numbers();
            assert_eq!(nb_of_matching_numbers, 4);
        }

        #[test]
        fn test_card_set() {
            aocstd::init_tests();

            let input_stream: Box<dyn std::io::BufRead> = Box::new(std::io::BufReader::new(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
                    .as_bytes(),
            ));
            let card_set = super::CardSet::from_input_stream(input_stream);
            // Test ruleset 1
            let nb_of_points_won = card_set.nb_of_points_won_with_ruleset1();
            assert_eq!(nb_of_points_won, 13);
            // Test ruleset 2
            let nb_of_cards_won = card_set.nb_of_cards_won_with_ruleset2();
            assert_eq!(nb_of_cards_won, 30);
        }
    }

    pub fn solve_part1(input_stream: Box<dyn BufRead>) {
        let card_set = CardSet::from_input_stream(input_stream);
        let nb_of_points_won = card_set.nb_of_points_won_with_ruleset1();
        log::info!("Part 1: {}", nb_of_points_won);
    }

    pub fn solve_part2(input_stream: Box<dyn BufRead>) {
        let card_set = CardSet::from_input_stream(input_stream);
        let nb_of_cards_won = card_set.nb_of_cards_won_with_ruleset2();
        log::info!("Part 2: {}", nb_of_cards_won);
    }
}

fn main() {
    let cli = aocstd::Cli::parse();
    aocstd::init_logger(&cli);
    let input_stream: Box<dyn BufRead> = aocstd::get_input_stream(&cli);

    match cli.part {
        aocstd::Part::Part1 => {
            scratchcards::solve_part1(input_stream);
        }
        aocstd::Part::Part2 => {
            scratchcards::solve_part2(input_stream);
        }
    }
}
