use crate::*;

// starts with red 32, ends with black 26
const NUMBER_MAPPING: [u8; 37] = [
    0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10, 5, 24, 16, 33, 1, 20,
    14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26,
];

#[near(serializers = [json, borsh])]
pub enum BetKind {
    // Inside Bets
    Straight, // Single number
    Split,
    Street,
    Corner,
    SixLine,   // Double Street
    Trio,      // 0-1-2 or 0-2-3
    FirstFour, // 0-1-2-3

    // Outside Bets
    Column,
    Dozen,
    Red,
    Black,
    Odd,
    Even,
    Low, // 1-18
    High, // 19-36

         // // Call (Announced) Bets
         // VoisinsDuZero,
         // TiersDuCylindre,
         // Orphelins,
         // Neighbors, // A number + neighbors
         // Finals,    // Numbers ending with same digit
}

#[near(serializers = [json, borsh])]
pub struct Bet {
    pub kind: BetKind,
    pub amount: NearToken,
    pub numbers: Vec<u8>,
}

pub fn bet_eval(rng_val: u8, bet: &Bet) -> (bool, u8, u8) {
    let index = rng_val % 37;
    let number = NUMBER_MAPPING[index as usize];
    let (win, multiple) = match bet.kind {
        BetKind::Straight => (number == bet.numbers[0], 35),
        BetKind::Black => (index != 0 && index % 2 == 0, 1),
        BetKind::Red => (index != 0 && index % 2 == 1, 1),
        _ => (false, 0),
    };

    if win {
        (win, number, multiple)
    } else {
        (false, number, 0)
    }
}
