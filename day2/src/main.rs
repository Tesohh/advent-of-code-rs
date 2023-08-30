use std::{env, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
struct Game {
    opponent: RPS,
    me: RPS,
}

impl Game {
    fn calculate(&self) -> u32 {
        let mut total = 0;

        total += match self.me {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };

        const WIN_VALUE: u32 = 6;
        total += match self.me {
            RPS::Rock if self.opponent == RPS::Scissors => WIN_VALUE,
            RPS::Paper if self.opponent == RPS::Rock => WIN_VALUE,
            RPS::Scissors if self.opponent == RPS::Paper => WIN_VALUE,
            _ if self.me == self.opponent => 3,
            _ => 0,
        };

        return total;
    }
}

#[derive(Debug, PartialEq)]
enum WhatToDo {
    Lose,
    Draw,
    Win,
}
trait OptimalMove {
    fn optimal_move(&self, opponent: RPS) -> RPS;
}
impl OptimalMove for WhatToDo {
    fn optimal_move(&self, opponent: RPS) -> RPS {
        match opponent {
            RPS::Rock if *self == WhatToDo::Win => RPS::Paper,
            RPS::Paper if *self == WhatToDo::Win => RPS::Scissors,
            RPS::Scissors if *self == WhatToDo::Win => RPS::Rock,

            RPS::Rock if *self == WhatToDo::Lose => RPS::Scissors,
            RPS::Paper if *self == WhatToDo::Lose => RPS::Rock,
            RPS::Scissors if *self == WhatToDo::Lose => RPS::Paper,

            _ => opponent, // in case we need to Draw
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = match args.get(1) {
        Some(a) => a,
        None => panic!("Please provide a file name"),
    };

    let file = match fs::read_to_string(arg) {
        Ok(result) => result,
        Err(..) => panic!("Cannot read file {}", arg),
    };

    let mut games: Vec<Game> = vec![];
    for line in file.lines() {
        const ERR_CANNOT_READ_FILE: &str = "Cannot read file correctly. format: A X";
        let encrypted_opponent_move = line
            .chars()
            .nth(0)
            .expect(ERR_CANNOT_READ_FILE)
            .to_ascii_uppercase();
        let encrypted_my_move = line
            .chars()
            .nth(2)
            .expect(ERR_CANNOT_READ_FILE)
            .to_ascii_uppercase();

        let opp = match encrypted_opponent_move {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            _ => panic!("invalid move! must be A, B, or C"),
        };

        let game = Game {
            opponent: opp,
            me: match encrypted_my_move {
                'X' => WhatToDo::Lose,
                'Y' => WhatToDo::Draw,
                'Z' => WhatToDo::Win,
                _ => panic!("invalid move! must be X, Y or Z"),
            }
            .optimal_move(opp),
        };
        games.push(game)
    }

    let mut total: u32 = 0;
    for g in &games {
        total += g.calculate();
    }

    println!("{:#?}", games);
    println!("{}", total)
}
