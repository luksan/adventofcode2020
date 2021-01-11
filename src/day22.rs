use crate::GroupBlankLine;
use itertools::Itertools;
use std::collections::VecDeque;

const INPUT_FILE: &str = "data/day22.txt";

type Players = Vec<Player>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Players {
    line_source.into_iter().group_by_blanks(parse_player)
}

fn parse_player<S: AsRef<str>>(iter: &mut dyn Iterator<Item = S>) -> Player {
    iter.next();
    Player {
        cards: iter.map(|s| s.as_ref().parse().unwrap()).collect(),
    }
}

type Card = usize;

#[derive(Debug, Clone)]
struct Player {
    cards: VecDeque<Card>,
}

impl Player {
    fn draw(&mut self) -> Card {
        self.cards.pop_front().unwrap()
    }

    fn receive(&mut self, card: Card) {
        self.cards.push_back(card)
    }
}

fn part1(input: &Players) -> usize {
    let mut players = (*input).clone();
    loop {
        if players.iter().any(|p| p.cards.is_empty()) {
            break;
        }
        let mut played = players.iter_mut().map(|p| p.draw()).collect_vec();
        let i = played
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)
            .map(|(i, _)| i)
            .unwrap();
        players[i].receive(played.remove(i));
        players[i].receive(played.remove(0));
    }

    players
        .iter()
        .flat_map(|p| p.cards.iter().rev())
        .enumerate()
        .fold(0, |score, (mult, &card)| score + (mult + 1) * card)
}

fn part2(_lines: &Players) -> usize {
    0
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 2);
    assert_eq!(part1(&d), 31308);
    // assert_eq!(part2(&d), 1);
}

#[test]
fn test_data() {
    let data = // Example data
"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    let d = load_input(data.lines());
    assert_eq!(part1(&d), 306);
    // assert_eq!(part2(&d), 1);
}
