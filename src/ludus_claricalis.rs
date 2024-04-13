use std::io::Write;

use itertools::Itertools;
use rand::seq::SliceRandom;

pub struct LudusClaricalis {
    dice: [u8; 3],
}

pub struct Output {
    pub roll: String,
    pub virtue: String,
}

impl LudusClaricalis {
    #[must_use]
    pub fn play(duration_ms: usize) -> Output {
        let ludus = Self::new();
        Self::roll_animation(duration_ms);

        Output {
            roll: ludus.roll_string(),
            virtue: ludus.virtue_string(),
        }
    }

    fn new() -> Self {
        Self::default()
    }

    fn from_roll(roll: [u8; 3]) -> Self {
        Self { dice: roll }
    }

    #[must_use]
    pub fn virtue_string(&self) -> String {
        match self.dice {
            [1, 1, 1] => "love",
            [1, 1, 2] => "faith",
            [1, 1, 3] => "hope",
            [1, 1, 4] => "justice",
            [1, 1, 5] => "prudence",
            [1, 1, 6] => "temperance",
            [1, 2, 2] => "courage",
            [1, 2, 3] => "peace",
            [1, 2, 4] => "chastity",
            [1, 2, 5] => "mercy",
            [1, 2, 6] => "obedience",
            [1, 3, 3] => "fear",
            [1, 3, 4] => "foresight",
            [1, 3, 5] => "discretion",
            [1, 3, 6] => "perseverance",
            [1, 4, 4] => "kindness",
            [1, 4, 5] => "modesty",
            [1, 4, 6] => "resignation",
            [1, 5, 5] => "gentleness",
            [1, 5, 6] => "generosity",
            [1, 6, 6] => "wisdom",
            [2, 2, 2] => "remorse",
            [2, 2, 3] => "joy",
            [2, 2, 4] => "sobriety",
            [2, 2, 5] => "satisfaction",
            [2, 2, 6] => "sweetness",
            [2, 3, 3] => "cleverness",
            [2, 3, 4] => "simplicity",
            [2, 3, 5] => "hospitality",
            [2, 3, 6] => "economy",
            [2, 4, 4] => "patience",
            [2, 4, 5] => "zeal",
            [2, 4, 6] => "poverty",
            [2, 5, 5] => "softness",
            [2, 5, 6] => "virginity",
            [2, 6, 6] => "respect",
            [3, 3, 3] => "piety",
            [3, 3, 4] => "indulgence",
            [3, 3, 5] => "prayer",
            [3, 3, 6] => "affection",
            [3, 4, 4] => "judgment",
            [3, 4, 5] => "vigilance",
            [3, 4, 6] => "mortification",
            [3, 5, 5] => "innocence",
            [3, 5, 6] => "contrition",
            [3, 6, 6] => "confession",
            [4, 4, 4] => "maturity",
            [4, 4, 5] => "solicitude",
            [4, 4, 6] => "constancy",
            [4, 5, 5] => "intelligence",
            [4, 5, 6] => "sighing",
            [4, 6, 6] => "weeping",
            [5, 5, 5] => "cheerfulness",
            [5, 5, 6] => "compassion",
            [5, 6, 6] => "self-control",
            [6, 6, 6] => "humility",
            _ => unreachable!("Impossible dice combination. Check that the dice are sorted."),
        }
        .to_string()
    }

    fn roll_combinations() -> Vec<[u8; 3]> {
        (1..=6)
            .combinations_with_replacement(3)
            .map(|v| [v[0], v[1], v[2]])
            .collect()
    }

    fn roll_permutations() -> Vec<[u8; 3]> {
        (1..=6)
            .permutations(3)
            .map(|v| [v[0], v[1], v[2]])
            .collect()
    }

    /// Like a slot machine, successively print random rolls and then wipe the line.
    /// duration_ms is the total duration of the animation in milliseconds.
    fn roll_animation(duration_ms: usize) {
        let mut rolls = Self::roll_permutations();
        let num_rolls = rolls.len();
        rolls.shuffle(&mut rand::thread_rng());

        let avg_delay = duration_ms as f64 / rolls.len() as f64;

        for (i, roll) in rolls.into_iter().enumerate() {
            let animation_fraction = i as f64 / num_rolls as f64;
            let delay_scale = 6.931_471_805_6 / 2.0f64.powf(10.0 - 10.0 * animation_fraction);
            let delay = std::time::Duration::from_millis((avg_delay * delay_scale) as u64);
            let game = Self::from_roll(roll);
            // print a bunch of spaces to clear the line
            print!("\r{}", game.roll_string());
            std::io::stdout().flush().unwrap();
            std::thread::sleep(delay);
            print!("\r{}", " ".repeat(30));
        }
        print!("\r");
    }

    #[must_use]
    pub fn roll_string(&self) -> String {
        roll_to_string(self.dice)
    }
}

impl Default for LudusClaricalis {
    fn default() -> Self {
        let roll = Self::roll_combinations()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_owned();

        Self {
            dice: [roll[0], roll[1], roll[2]],
        }
    }
}

fn display_die(die: u8) -> String {
    match die {
        1 => "⚀".to_string(),
        2 => "⚁".to_string(),
        3 => "⚂".to_string(),
        4 => "⚃".to_string(),
        5 => "⚄".to_string(),
        6 => "⚅".to_string(),
        _ => unreachable!("Invalid die value"),
    }
}

fn roll_to_string(roll: [u8; 3]) -> String {
    roll.iter().map(|&die| display_die(die)).join("")
}
