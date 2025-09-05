use contract_rs::roulette::*;

use near_sdk::NearToken;
use rand::Rng;
// Probability Test
#[test]
fn test_straight_bet_probability() {
    let mut rng = rand::rng();
    let mut wins = 0;
    let total = 1_000_000;
    let my_number = 17; // Choosing "17" arbitrarily (any 0..=36 is fine)
    let bet = Bet {
        kind: BetKind::Straight,
        amount: NearToken::from_millinear(100),
        number: my_number,
    };

    for _ in 0..total {
        let spin = rng.random_range(0..=255) as u8; // simulate full u8 as per code
        let (win, _, _, _) = bet_eval(spin, &bet);
        if win {
            wins += 1;
        }
    }

    let probability = wins as f64 / total as f64;

    println!(
        "Split: wins={}, prob={:.5}, exp={:.5}",
        wins,
        probability,
        1.0 / 37.0
    );

    let allowed_delta = 0.004;
    assert!(
        (probability - (1.0 / 37.0)).abs() < allowed_delta,
        "Empirical: {probability}, Expected: {:.4}",
        1.0 / 37.0
    );
}

#[test]
fn test_split_bet_probability() {
    let bet_count = SPLIT_BETS.len();
    let total_per_bet = 17_544; // 57 * 17_544 = 999_978 (~1 million)
    let total_trials = bet_count * total_per_bet;

    let mut rng = rand::rng();
    let mut total_wins = 0u64;

    for bet_index in 0..bet_count {
        let bet = Bet {
            kind: BetKind::Split,
            amount: NearToken::from_millinear(100),
            number: bet_index as u8, // index into SPLIT_BETS
        };
        let mut bet_wins = 0;
        for _ in 0..total_per_bet {
            let spin = rng.random_range(0..=255) as u8; // covers the wheel like original
            let (win, _, _, _) = bet_eval(spin, &bet);
            if win {
                bet_wins += 1;
            }
        }
        total_wins += bet_wins;
        // Optional: print individual bet stats
        // println!("Split #{}: wins={}, prob={:.5}", bet_index, bet_wins, bet_wins as f64 / total_per_bet as f64);
    }

    let empirical_prob = total_wins as f64 / total_trials as f64;
    let expected = 2.0 / 37.0;

    println!(
        "Split: wins={}, prob={:.5}, exp={:.5}",
        total_wins, empirical_prob, expected
    );
    let allowed_delta = 0.004;
    assert!(
        (empirical_prob - expected).abs() < allowed_delta,
        "Empirical: {:.5}, Expected: {:.5}",
        empirical_prob,
        expected
    );
}

#[test]
fn test_street_bet_probability() {
    let bet_count = STREET_BETS.len();
    let total_per_bet = 83_334; // 12 * 83_334 = 1,000,008 (~1 million)
    let total_trials = bet_count * total_per_bet;

    let mut rng = rand::rng();
    let mut total_wins = 0u64;

    for bet_index in 0..bet_count {
        let bet = Bet {
            kind: BetKind::Street,
            amount: NearToken::from_millinear(100),
            number: bet_index as u8, // index into STREET_BETS
        };
        let mut bet_wins = 0;
        for _ in 0..total_per_bet {
            let spin = rng.random_range(0..=255) as u8;
            let (win, _, _, _) = bet_eval(spin, &bet);
            if win {
                bet_wins += 1;
            }
        }
        total_wins += bet_wins;
        // Optional: print individual bet stats
        // println!("Street #{}: wins={}, prob={:.5}", bet_index, bet_wins, bet_wins as f64 / total_per_bet as f64);
    }

    let empirical_prob = total_wins as f64 / total_trials as f64;
    let expected = 3.0 / 37.0;

    println!(
        "Steet: wins={}, prob={:.5}, exp={:.5}",
        total_wins, empirical_prob, expected
    );
    let allowed_delta = 0.004;
    assert!(
        (empirical_prob - expected).abs() < allowed_delta,
        "Empirical: {:.5}, Expected: {:.5}",
        empirical_prob,
        expected
    );
}

#[test]
fn test_corner_bet_probability() {
    let bet_count = CORNER_BETS.len();
    let total_per_bet = 45_455; // 22 * 45_455 = 999,910 (~1 million)
    let total_trials = bet_count * total_per_bet;

    let mut rng = rand::rng();
    let mut total_wins = 0u64;

    for bet_index in 0..bet_count {
        let bet = Bet {
            kind: BetKind::Corner,
            amount: NearToken::from_millinear(100),
            number: bet_index as u8, // index into CORNER_BETS
        };
        let mut bet_wins = 0;
        for _ in 0..total_per_bet {
            let spin = rng.random_range(0..=255) as u8;
            let (win, _, _, _) = bet_eval(spin, &bet);
            if win {
                bet_wins += 1;
            }
        }
        total_wins += bet_wins;
        // Optional: print for individual corners:
        // println!("Corner #{}: wins={}, p={:.5}", bet_index, bet_wins, bet_wins as f64 / total_per_bet as f64);
    }

    let empirical_prob = total_wins as f64 / total_trials as f64;
    let expected = 4.0 / 37.0;
    println!(
        "Corner: wins={}, prob={:.5}, exp={:.5}",
        total_wins, empirical_prob, expected
    );
    let allowed_delta = 0.004; // simulation margin
    assert!(
        (empirical_prob - expected).abs() < allowed_delta,
        "Empirical: {:.5}, Expected: {:.5}",
        empirical_prob,
        expected
    );
}

#[test]
fn test_six_line_bet_probability() {
    let bet_count = SIX_LINE_BETS.len();
    let total_per_bet = 90_910; // 11 * 90_910 = 999,910 (~1 million)
    let total_trials = bet_count * total_per_bet;

    let mut rng = rand::rng();
    let mut total_wins = 0u64;

    for bet_index in 0..bet_count {
        let bet = Bet {
            kind: BetKind::SixLine,
            amount: NearToken::from_millinear(100),
            number: bet_index as u8, // index into SIX_LINE_BETS
        };
        let mut bet_wins = 0;
        for _ in 0..total_per_bet {
            let spin = rng.random_range(0..=255) as u8;
            let (win, _, _, _) = bet_eval(spin, &bet);
            if win {
                bet_wins += 1;
            }
        }
        total_wins += bet_wins;
        // Optional: println!("SixLine #{}: wins={}, p={:.5}", bet_index, bet_wins, bet_wins as f64 / total_per_bet as f64);
    }

    let empirical_prob = total_wins as f64 / total_trials as f64;

    let expected = 6.0 / 37.0;
    println!(
        "Six Line: wins={}, prob={:.5}, exp={:.5}",
        total_wins, empirical_prob, expected
    );
    let allowed_delta = 0.004;
    assert!(
        (empirical_prob - expected).abs() < allowed_delta,
        "Empirical: {:.5}, Expected: {:.5}",
        empirical_prob,
        expected
    );
}

#[test]
fn test_column_and_dozen_bet_probability() {
    let trials = 1_000_000;

    let mut rng = rand::rng();
    let mut column_wins = 0u64;
    let mut dozen_wins = 0u64;

    // Test columns
    for _ in 0..trials {
        let column = rng.random_range(0..=2) as u8; // random column: 0,1,2
        let bet = Bet {
            kind: BetKind::Column,
            amount: NearToken::from_millinear(100),
            number: column,
        };
        let spin = rng.random_range(0..=255) as u8;
        let (win, _, _, _) = bet_eval(spin, &bet);
        if win {
            column_wins += 1;
        }
    }

    // Test dozens
    for _ in 0..trials {
        let dozen = rng.random_range(0..=2) as u8; // random dozen: 0,1,2
        let bet = Bet {
            kind: BetKind::Dozen,
            amount: NearToken::from_millinear(100),
            number: dozen,
        };
        let spin = rng.random_range(0..=255) as u8;
        let (win, _, _, _) = bet_eval(spin, &bet);
        if win {
            dozen_wins += 1;
        }
    }

    let column_prob = column_wins as f64 / trials as f64;
    let dozen_prob = dozen_wins as f64 / trials as f64;

    let expected = 12.0 / 37.0;

    println!(
        "Column: wins={}, prob={:.5}, exp={:.5}",
        column_wins, column_prob, expected
    );
    println!(
        "Dozen: wins={}, prob={:.5}, exp={:.5}",
        dozen_wins, dozen_prob, expected
    );

    let allowed_delta = 0.004;
    assert!(
        (column_prob - expected).abs() < allowed_delta,
        "Column Empirical: {:.5}, Expected: {:.5}",
        column_prob,
        expected
    );
    assert!(
        (dozen_prob - expected).abs() < allowed_delta,
        "Dozen Empirical: {:.5}, Expected: {:.5}",
        dozen_prob,
        expected
    );
}

#[test]
fn test_even_money_bets_probability() {
    let mut rng = rand::rng();
    let bet_types: [u8; 6] = [0, 1, 2, 3, 4, 5];
    let labels = ["Red", "Black", "Odd", "Even", "Low", "High"];
    let expected = 18.0 / 37.0;
    let allowed_delta = 0.004;
    const TRIALS: usize = 1_000_000;

    for (i, bet_kind) in bet_types.iter().enumerate() {
        let mut wins = 0u64;
        // Clone or dereference as needed; here slice yields &BetKind so we use .clone()
        let kind = match bet_kind {
            0 => BetKind::Red,
            1 => BetKind::Black,
            2 => BetKind::Odd,
            3 => BetKind::Even,
            4 => BetKind::Low,
            5 => BetKind::High,
            _ => BetKind::Red,
        };
        let bet = Bet {
            kind,
            amount: NearToken::from_millinear(100),
            number: 0,
        };
        for _ in 0..TRIALS {
            let spin = rng.random_range(0..=255) as u8;
            let (win, _, _, _) = bet_eval(spin, &bet);
            if win {
                wins += 1;
            }
        }
        let prob = wins as f64 / TRIALS as f64;
        println!(
            "{}: wins={}, prob={:.5}, exp={:.5}",
            labels[i], wins, prob, expected
        );
        assert!(
            (prob - expected).abs() < allowed_delta,
            "{}: Empirical={:.5}, Expected={:.5}",
            labels[i],
            prob,
            expected
        );
    }
}
