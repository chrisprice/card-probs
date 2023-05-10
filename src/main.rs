use std::mem::swap;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Artist {
    Monet,
    DaVinci,
    VanGogh,
    Picasso,
    Rembrandt,
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Card {
    WaterLilies,
    MonaLisa,
    StarryNight,
    Guernica,
    BlueNude,
    TheNightWatch,
    ThePotatoEaters,
    Haystacks,
    Sunflowers,
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WaterLilies => write!(
                f,
                "WaterLilies ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::MonaLisa => write!(
                f,
                "MonaLisa ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::StarryNight => write!(
                f,
                "StarryNight ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::Guernica => write!(
                f,
                "Guernica ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::BlueNude => write!(
                f,
                "BlueNude ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::TheNightWatch => write!(
                f,
                "TheNightWatch ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::ThePotatoEaters => write!(
                f,
                "ThePotatoEaters ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::Haystacks => write!(
                f,
                "Haystacks ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
            Self::Sunflowers => write!(
                f,
                "Sunflowers ({:?}, {:?}, masterpiece: {:?}, early_work: {:?})",
                self.artist(),
                self.value(),
                self.is_masterpiece(),
                self.is_early_work()
            ),
        }
    }
}

impl Card {
    fn artist(&self) -> Artist {
        match self {
            Card::WaterLilies | Card::Haystacks => Artist::Monet,
            Card::MonaLisa => Artist::DaVinci,
            Card::StarryNight | Card::ThePotatoEaters | Card::Sunflowers => Artist::VanGogh,
            Card::Guernica | Card::BlueNude => Artist::Picasso,
            Card::TheNightWatch => Artist::Rembrandt,
        }
    }

    fn value(&self) -> u8 {
        match self {
            Card::WaterLilies | Card::MonaLisa => 4,
            Card::StarryNight | Card::Guernica | Card::BlueNude => 3,
            Card::TheNightWatch | Card::ThePotatoEaters | Card::Haystacks | Card::Sunflowers => 2,
        }
    }

    fn is_masterpiece(&self) -> bool {
        match self {
            Card::MonaLisa | Card::Guernica | Card::Sunflowers | Card::TheNightWatch => true,
            _ => false,
        }
    }

    fn is_early_work(&self) -> bool {
        match self {
            Card::BlueNude | Card::ThePotatoEaters => true,
            _ => false,
        }
    }

    fn is_pull_alarm(&self) -> bool {
        match self {
            Card::TheNightWatch | Card::ThePotatoEaters | Card::Sunflowers => true,
            _ => false,
        }
    }

    fn new_deck() -> [Card; 9] {
        let mut deck = [
            Card::WaterLilies,
            Card::MonaLisa,
            Card::StarryNight,
            Card::Guernica,
            Card::BlueNude,
            Card::TheNightWatch,
            Card::ThePotatoEaters,
            Card::Haystacks,
            Card::Sunflowers,
        ];
        let mut rng = thread_rng();
        deck.shuffle(&mut rng);
        deck
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Player {
    One,
    Two,
}

impl Player {
    fn next(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Score {
    score: u8,
    max_masterpiece_score: u8,
    masterpiece_count: u8,
    max_early_work_score: u8,
    early_work_count: u8,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Hand(Card, Card);

impl Hand {
    fn calculate_score(&self) -> Score {
        let Hand(a, b) = self;
        let mut score = a.value() + b.value();
        if a.artist() == b.artist() {
            score += 2;
        }
        let max_masterpiece_score = match (a.is_masterpiece(), b.is_masterpiece()) {
            (true, true) => a.value().max(b.value()),
            (true, false) => a.value(),
            (false, true) => b.value(),
            (false, false) => 0,
        };
        let max_early_work_score = match (a.is_early_work(), b.is_early_work()) {
            (true, true) => a.value().max(b.value()),
            (true, false) => a.value(),
            (false, true) => b.value(),
            (false, false) => 0,
        };
        let masterpiece_count = a.is_masterpiece() as u8 + b.is_masterpiece() as u8;
        let early_work_count = a.is_early_work() as u8 + b.is_early_work() as u8;
        Score {
            score,
            max_masterpiece_score,
            masterpiece_count,
            max_early_work_score,
            early_work_count,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct GameState {
    player_one_hand: Hand,
    player_two_hand: Hand,
    player_to_play: Player,
    gallery_deck: Vec<Card>,
    private_collection_deck: Vec<Card>,
}

impl GameState {
    fn new([a, b, c, d, e, f, g, h, i]: [Card; 9]) -> Self {
        let mut gallery_deck = Vec::with_capacity(5);
        gallery_deck.push(e);
        GameState {
            player_one_hand: Hand(a, b),
            player_two_hand: Hand(c, d),
            player_to_play: Player::One,
            gallery_deck,
            private_collection_deck: vec![f, g, h, i],
        }
    }
    fn play_next_move(&mut self) -> Option<Player> {
        let mut rng = thread_rng();
        let player_deck = match self.player_to_play {
            Player::One => &mut self.player_one_hand,
            Player::Two => &mut self.player_two_hand,
        };
        self.player_to_play = self.player_to_play.next();
        if rng.gen_bool(0.1) && (player_deck.0.is_pull_alarm() || player_deck.1.is_pull_alarm()) {
            return Some(self.calculate_winner());
        }
        let card_to_play = if rng.gen_bool(0.5) {
            &mut player_deck.0
        } else {
            &mut player_deck.1
        };
        if rng.gen_bool(0.5) {
            let top_card = self
                .gallery_deck
                .get_mut(0)
                .expect("gallery deck to always contain at least one card");
            swap(card_to_play, top_card);
        } else {
            let mut top_card = self
                .private_collection_deck
                .pop()
                .expect("private collection deck to always contain at least one card");
            swap(card_to_play, &mut top_card);
            self.gallery_deck.push(top_card);
            if self.private_collection_deck.is_empty() {
                return Some(self.calculate_winner());
            }
        }
        None
    }

    fn calculate_winner(&self) -> Player {
        let player_one = self.player_one_hand.calculate_score();
        let player_two = self.player_two_hand.calculate_score();

        if player_one.score > player_two.score {
            Player::One
        } else if player_two.score > player_one.score {
            Player::Two
        } else {
            if player_one.masterpiece_count > player_two.masterpiece_count {
                Player::One
            } else if player_two.masterpiece_count > player_one.masterpiece_count {
                Player::Two
            } else {
                if player_one.early_work_count < player_two.early_work_count {
                    Player::One
                } else if player_two.early_work_count < player_one.early_work_count {
                    Player::Two
                } else {
                    if player_one.max_masterpiece_score > player_two.max_masterpiece_score {
                        Player::One
                    } else if player_two.max_masterpiece_score > player_one.max_masterpiece_score {
                        Player::Two
                    } else {
                        if player_one.max_early_work_score > player_two.max_early_work_score {
                            Player::One
                        } else if player_two.max_early_work_score > player_one.max_early_work_score
                        {
                            Player::Two
                        } else {
                            println!("Game state {:#?}", self);
                            unreachable!("There should never be a tie")
                        }
                    }
                }
            }
        }
        // match winner {
        //     Player::One => (player_one.score, player_two.masterpiece_bonus),
        //     Player::Two => (player_one.masterpiece_bonus, player_two.score),
        // }
    }
}

fn main() {
    let mut player_1_wins = 0;
    let mut player_2_wins = 0;
    let mut stalemates = 0;
    for i in 0..1_000_000 {
        if i % 100_000 == 0 {
            println!("Game {}", i);
        }
        let deck = Card::new_deck();
        let mut game_state = GameState::new(deck);
        // println!("Game state {:#?}", game_state);
        let mut winner = None;
        let mut turns_remaining = 16;
        while turns_remaining > 0 && winner.is_none() {
            turns_remaining -= 1;
            winner = game_state.play_next_move();
        }
        match winner {
            Some(Player::One) => player_1_wins += 1,
            Some(Player::Two) => player_2_wins += 1,
            None => stalemates += 1,
        }
    }
    println!(
        "Player 1 wins: {}, Player 2 wins: {}, Stalemates: {}",
        player_1_wins, player_2_wins, stalemates
    )
}
