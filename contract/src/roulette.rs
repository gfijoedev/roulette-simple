use crate::*;

#[derive(Debug)]
#[near(serializers = [json, borsh])]
pub enum BetKind {
    // Inside Bets
    Straight,
    Split,
    Street,
    Corner,
    SixLine,
    // Outside Bets
    Column,
    Dozen,
    Red,
    Black,
    Odd,
    Even,
    Low,
    High,
}

#[derive(Debug)]
#[near(serializers = [json, borsh])]
pub struct Bet {
    pub kind: BetKind,
    pub amount: NearToken,
    pub number: u8,
}

pub fn bet_legal(bet: &Bet) -> bool {
    match bet.kind {
        BetKind::Straight => bet.number > 0 && bet.number < 37,
        BetKind::Split => bet.number < SPLIT_BETS.len() as u8, // index of SPLIT_BETS
        BetKind::Street => bet.number < STREET_BETS.len() as u8, // ..
        BetKind::Corner => bet.number < CORNER_BETS.len() as u8, // ..
        BetKind::SixLine => bet.number < SIX_LINE_BETS.len() as u8, // ..
        BetKind::Column => bet.number < 3,                     // calc manually
        BetKind::Dozen => bet.number < 3,                      // calc manually
        BetKind::Black => true,
        BetKind::Red => true,
        BetKind::Odd => true,
        BetKind::Even => true,
        BetKind::Low => true,
        BetKind::High => true,
    }
}

pub fn bet_eval(rng_val: u8, bet: &Bet) -> (bool, u8, bool, u8) {
    let index = rng_val % 37;
    let number = WHEEL_MAPPING[index as usize];
    let red: bool = index % 2 == 1;
    let (win, multiple) = match bet.kind {
        BetKind::Straight => (bet.number == number, 35),
        BetKind::Split => {
            let (a, b) = SPLIT_BETS[bet.number as usize];
            (number == a || number == b, 17)
        }
        BetKind::Street => {
            let (a, b, c) = STREET_BETS[bet.number as usize];
            let numbers = vec![a, b, c];
            (numbers.contains(&number), 11)
        }
        BetKind::Corner => {
            let (a, b, c, d) = CORNER_BETS[bet.number as usize];
            let numbers = vec![a, b, c, d];
            (numbers.contains(&number), 8)
        }
        BetKind::SixLine => {
            let (a, b, c, d, e, f) = SIX_LINE_BETS[bet.number as usize];
            let numbers = vec![a, b, c, d, e, f];
            (numbers.contains(&number), 5)
        }
        BetKind::Column => (number > 0 && (number - 1) % 3 == bet.number, 2),
        BetKind::Dozen => (number > 0 && (number - 1) / 12 == bet.number, 2),
        BetKind::Black => (number > 0 && !red, 1),
        BetKind::Red => (number > 0 && red, 1),
        BetKind::Odd => (number > 0 && number % 2 == 1, 1),
        BetKind::Even => (number > 0 && number % 2 == 0, 1),
        BetKind::Low => (number > 0 && number < 19, 1),
        BetKind::High => (number > 18, 1),
    };

    if win {
        (win, number, red, multiple)
    } else {
        (false, number, red, 0)
    }
}

// consts for wheel, bet index to numbers

// starts with red 32, ends with black 26
pub const WHEEL_MAPPING: [u8; 37] = [
    0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5, 24, 16, 33, 1, 20,
    14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26,
];

/// All valid split pairs, order: left-right, top-bottom (European 3-column table)
pub const SPLIT_BETS: [(u8, u8); 57] = [
    // Row 1 (1-2-3)
    (1, 2),
    (2, 3),
    (1, 4),
    (2, 5),
    (3, 6),
    // Row 2 (4-5-6)
    (4, 5),
    (5, 6),
    (4, 7),
    (5, 8),
    (6, 9),
    // Row 3 (7-8-9)
    (7, 8),
    (8, 9),
    (7, 10),
    (8, 11),
    (9, 12),
    // Row 4 (10-11-12)
    (10, 11),
    (11, 12),
    (10, 13),
    (11, 14),
    (12, 15),
    // Row 5 (13-14-15)
    (13, 14),
    (14, 15),
    (13, 16),
    (14, 17),
    (15, 18),
    // Row 6 (16-17-18)
    (16, 17),
    (17, 18),
    (16, 19),
    (17, 20),
    (18, 21),
    // Row 7 (19-20-21)
    (19, 20),
    (20, 21),
    (19, 22),
    (20, 23),
    (21, 24),
    // Row 8 (22-23-24)
    (22, 23),
    (23, 24),
    (22, 25),
    (23, 26),
    (24, 27),
    // Row 9 (25-26-27)
    (25, 26),
    (26, 27),
    (25, 28),
    (26, 29),
    (27, 30),
    // Row 10 (28-29-30)
    (28, 29),
    (29, 30),
    (28, 31),
    (29, 32),
    (30, 33),
    // Row 11 (31-32-33)
    (31, 32),
    (32, 33),
    (31, 34),
    (32, 35),
    (33, 36),
    // Row 12 (34-35-36)
    (34, 35),
    (35, 36),
];

pub const STREET_BETS: [(u8, u8, u8); 12] = [
    (1, 2, 3),
    (4, 5, 6),
    (7, 8, 9),
    (10, 11, 12),
    (13, 14, 15),
    (16, 17, 18),
    (19, 20, 21),
    (22, 23, 24),
    (25, 26, 27),
    (28, 29, 30),
    (31, 32, 33),
    (34, 35, 36),
];

pub const CORNER_BETS: [(u8, u8, u8, u8); 22] = [
    (1, 2, 4, 5),
    (2, 3, 5, 6),
    (4, 5, 7, 8),
    (5, 6, 8, 9),
    (7, 8, 10, 11),
    (8, 9, 11, 12),
    (10, 11, 13, 14),
    (11, 12, 14, 15),
    (13, 14, 16, 17),
    (14, 15, 17, 18),
    (16, 17, 19, 20),
    (17, 18, 20, 21),
    (19, 20, 22, 23),
    (20, 21, 23, 24),
    (22, 23, 25, 26),
    (23, 24, 26, 27),
    (25, 26, 28, 29),
    (26, 27, 29, 30),
    (28, 29, 31, 32),
    (29, 30, 32, 33),
    (31, 32, 34, 35),
    (32, 33, 35, 36),
];

pub const SIX_LINE_BETS: [(u8, u8, u8, u8, u8, u8); 11] = [
    (1, 2, 3, 4, 5, 6),
    (4, 5, 6, 7, 8, 9),
    (7, 8, 9, 10, 11, 12),
    (10, 11, 12, 13, 14, 15),
    (13, 14, 15, 16, 17, 18),
    (16, 17, 18, 19, 20, 21),
    (19, 20, 21, 22, 23, 24),
    (22, 23, 24, 25, 26, 27),
    (25, 26, 27, 28, 29, 30),
    (28, 29, 30, 31, 32, 33),
    (31, 32, 33, 34, 35, 36),
];
