use crate::GroupBlankLine;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::convert::TryInto;
use std::hash::{BuildHasherDefault, Hash, Hasher};

const INPUT_FILE: &str = "data/2020/day22.txt";

type Card = u8;
type Player = VecDeque<Card>;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Vec<Player> {
    line_source.into_iter().group_by_blanks(parse_player)
}

fn parse_player<S: AsRef<str>>(iter: &mut dyn Iterator<Item = S>) -> Player {
    iter.next();
    iter.map(|s| s.as_ref().parse().unwrap()).collect()
}

fn highest_card_idx(cards: &[Card; 2]) -> Winner {
    if cards[0] > cards[1] {
        0
    } else {
        1
    }
}

type Winner = usize;

#[derive(Hash)]
struct GameState([VecDeque<Card>; 2]);

impl GameState {
    fn from_input(input: &[Player]) -> GameState {
        GameState([input[0].clone(), input[1].clone()])
    }

    fn state_hash(&self) -> u64 {
        let hs = &mut DefaultHasher::new();
        self.hash(hs);
        hs.finish()
    }

    fn draw(&mut self) -> [Card; 2] {
        [
            self.0[0].pop_front().unwrap(),
            self.0[1].pop_front().unwrap(),
        ]
    }

    fn receive_cards(&mut self, winner: Winner, mut cards: [Card; 2]) {
        cards.swap(0, winner);
        self.0[winner].extend(&cards);
    }

    fn check_empty_hand(&self) -> Option<Winner> {
        match (self.0[0].is_empty(), self.0[1].is_empty()) {
            (true, false) => Some(1),
            (false, true) => Some(0),
            (false, false) => None,
            _ => unreachable!("The cards have dissapeared!"),
        }
    }

    fn final_result(self) -> usize {
        self.0
            .iter()
            .flat_map(|p| p.iter().rev())
            .enumerate()
            .fold(0, |score, (mult, card)| {
                score + (mult + 1) * (*card as usize)
            })
    }

    fn sub_game(&self, card_cnt: &[Card; 2]) -> Option<GameState> {
        let p = &self.0;
        if p[0].len() < card_cnt[0] as usize || p[1].len() < card_cnt[1] as usize {
            return None;
        }
        Some(GameState([
            self.0[0]
                .iter()
                .take(card_cnt[0] as usize)
                .copied()
                .collect(),
            self.0[1]
                .iter()
                .take(card_cnt[1] as usize)
                .copied()
                .collect(),
        ]))
    }
}

fn part1(input: &[Player]) -> usize {
    let mut gs = GameState::from_input(input);
    while gs.check_empty_hand().is_none() {
        let played = gs.draw();
        gs.receive_cards(highest_card_idx(&played), played);
    }
    gs.final_result()
}

#[derive(Default)]
struct SelfIsHash(u64);

impl Hasher for SelfIsHash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_le_bytes(bytes.try_into().unwrap())
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i
    }
}

fn recursive_combat(gs: &mut GameState) -> Winner {
    let mut history = HashSet::<u64, BuildHasherDefault<SelfIsHash>>::default();
    while history.insert(gs.state_hash()) {
        if let Some(winner) = gs.check_empty_hand() {
            return winner;
        }
        let played = gs.draw();
        let round_winner = match gs.sub_game(&played) {
            Some(mut sub_game) => recursive_combat(&mut sub_game),
            None => highest_card_idx(&played),
        };
        gs.receive_cards(round_winner, played);
    }
    0 // Repeated state is a win for player 1
}

fn part2(input: &[Player]) -> usize {
    let mut gs = GameState::from_input(input);
    recursive_combat(&mut gs);
    gs.final_result()
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(INPUT_FILE));
    assert_eq!(d.len(), 2);
    assert_eq!(part1(&d), 31308);
    assert_eq!(part2(&d), 33647);
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
    assert_eq!(part2(&d), 291);
}
